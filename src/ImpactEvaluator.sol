// SPDX-License-Identifier: (MIT or Apache-2.0)

import "../lib/openzeppelin-contracts/contracts/access/AccessControl.sol";
import "../lib/openzeppelin-contracts/contracts/utils/Nonces.sol";
import "../lib/openzeppelin-contracts/contracts/utils/cryptography/ECDSA.sol";
pragma solidity ^0.8.19;

contract ImpactEvaluator is AccessControl, Nonces {
    struct Round {
        uint index;
        uint end;
        uint totalScores;
        uint roundReward;
    }

    Round public currentRound;
    Round public previousRound;
    uint public nextRoundLength = 10;
    uint public roundReward = 100 ether;
    uint64 public constant MAX_SCORE = 1e15;
    mapping(address => uint) public balances;
    uint public balanceHeld = 0;

    event MeasurementsAdded(
        string cid,
        uint roundIndex,
        address indexed sender
    );
    event RoundStart(uint roundIndex);
    event Withdrawal(address indexed account, address target, uint256 value);

    bytes32 public constant EVALUATE_ROLE = keccak256("EVALUATE_ROLE");

    constructor(address admin) {
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(EVALUATE_ROLE, admin);
        advanceRound();
    }

    receive() external payable {}

    function advanceRound() private {
        uint availableInContract = address(this).balance - balanceHeld;
        uint nextAvailableRoundReward = availableInContract < roundReward
            ? availableInContract
            : roundReward;
        balanceHeld += nextAvailableRoundReward;
        previousRound = currentRound;
        currentRound = Round(
            currentRound.end == 0 ? 0 : currentRound.index + 1,
            block.number + nextRoundLength,
            0,
            nextAvailableRoundReward
        );
        emit RoundStart(currentRound.index);
    }

    function adminAdvanceRound() public onlyAdmin {
        advanceRound();
    }

    function setNextRoundLength(uint _nextRoundLength) public onlyAdmin {
        require(_nextRoundLength > 0, "Next round length must be positive");
        nextRoundLength = _nextRoundLength;
    }

    function setRoundReward(uint _roundReward) public onlyAdmin {
        roundReward = _roundReward;
    }

    function addMeasurements(string memory cid) public virtual returns (uint) {
        uint measurementsRoundIndex = currentRound.index;
        emit MeasurementsAdded(cid, measurementsRoundIndex, msg.sender);
        if (block.number >= currentRound.end) {
            advanceRound();
        }
        return measurementsRoundIndex;
    }

    function setScores(
        uint roundIndex,
        address payable[] memory addresses,
        uint64[] memory scores
    ) public onlyEvaluator {
        require(
            addresses.length == scores.length,
            "Addresses and scores length mismatch"
        );
        require(
            previousRound.index != currentRound.index &&
            roundIndex == previousRound.index,
            "Can only score previous round"
        );

        uint sumOfScores = validateScores(scores);
        reward(addresses, scores);
        previousRound.totalScores += sumOfScores;
    }

    function validateScores(uint64[] memory scores) public view returns (uint) {
        uint64 sum = 0;
        for (uint i = 0; i < scores.length; i++) {
            sum += scores[i];
        }
        require(sum <= MAX_SCORE, "Sum of scores too big");
        require(
            sum + previousRound.totalScores <= MAX_SCORE,
            "Sum of scores including historic too big"
        );
        return sum;
    }

    function reward(
        address payable[] memory addresses,
        uint64[] memory scores
    ) private {
        for (uint i = 0; i < addresses.length; i++) {
            address payable participant = addresses[i];
            uint amount = (scores[i] * previousRound.roundReward) / MAX_SCORE;
            if (participant == 0x000000000000000000000000000000000000dEaD) {
                balanceHeld -= amount;
            } else {
                balances[participant] += amount;
            }
        }
    }

    function balanceOf(address account) public view returns (uint) {
        return balances[account];
    }

    function _withdraw(
        address account,
        address payable target,
        uint value
    ) private {
        require(balances[account] >= value, "Insufficient balance");
        balances[account] -= value;
        balanceHeld -= value;
        if (balances[account] == 0) {
            delete balances[account];
        }
        require(target.send(value), "Withdrawal failed");
    }

    function withdraw(address payable target, uint value) public {
        _withdraw(msg.sender, target, value);
        emit Withdrawal(msg.sender, target, value);
    }

    function withdrawOnBehalf(
        address account,
        address payable target,
        uint value,
        uint8 v,
        bytes32 r,
        bytes32 s
    ) public {
        bytes32 digest = keccak256(
            abi.encode(account, _useNonce(account), msg.sender, target, value)
        );
        address signer = ECDSA.recover(digest, v, r, s);
        require(signer == account, "Invalid signature");

        _withdraw(account, payable(msg.sender), 0.1 ether);
        _withdraw(account, target, value - 0.1 ether);
        emit Withdrawal(account, target, value - 0.1 ether);
    }

    function currentRoundIndex() public view returns (uint) {
        return currentRound.index;
    }

    function currentRoundEnd() public view returns (uint) {
        return currentRound.end;
    }

    modifier onlyAdmin() {
        require(hasRole(DEFAULT_ADMIN_ROLE, msg.sender), "Not an admin");
        _;
    }

    modifier onlyEvaluator() {
        require(hasRole(EVALUATE_ROLE, msg.sender), "Not an evaluator");
        _;
    }
}
