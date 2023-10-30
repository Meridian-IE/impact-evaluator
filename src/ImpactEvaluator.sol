// SPDX-License-Identifier: (MIT or Apache-2.0)

import "../lib/openzeppelin-contracts/contracts/access/AccessControl.sol";
pragma solidity ^0.8.19;

contract ImpactEvaluator is AccessControl {
    struct Round {
        uint index;
        uint totalScores;
    }

    Round[] public openRounds;
    uint public nextRoundLength = 10;
    uint public roundReward = 100 ether;
    uint64 public constant MAX_SCORE = 1e15;
    uint public currentRoundIndex;
    uint public currentRoundEnd;
    mapping(address => uint) public balances;

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
        uint nextRoundIndex = openRounds.length == 0
            ? 0
            : currentRoundIndex + 1;
        Round memory round = Round(nextRoundIndex, 0);
        currentRoundEnd = block.number + nextRoundLength;
        currentRoundIndex = nextRoundIndex;
        openRounds.push(round);
        emit RoundStart(nextRoundIndex);
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
        if (block.number >= currentRoundEnd) {
            advanceRound();
        }
        emit MeasurementsAdded(cid, currentRoundIndex, msg.sender);
        return currentRoundIndex;
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
        require(roundIndex < currentRoundIndex, "Round not finished");

        (uint openRoundsIndex, Round storage round) = getOpenRound(roundIndex);
        uint sumOfScores = validateScores(scores, round.totalScores);
        reward(addresses, scores);
        round.totalScores += sumOfScores;

        if (round.totalScores == MAX_SCORE) {
            deleteOpenRound(openRoundsIndex);
        }
    }

    function getOpenRound(
        uint roundIndex
    ) private view returns (uint index, Round storage) {
        for (uint i = 0; i < openRounds.length; i++) {
            if (openRounds[i].index == roundIndex) {
                return (i, openRounds[i]);
            }
        }
        revert("Open round does not exist");
    }

    function deleteOpenRound(uint openRoundsIndex) private {
        // Remove the round while shrinking the array.
        for (uint i = openRoundsIndex; i < openRounds.length - 1; i++) {
            openRounds[i] = openRounds[i + 1];
        }
        openRounds.pop();
    }

    function adminDeleteOpenRound(uint roundIndex) public onlyAdmin {
        require(roundIndex < currentRoundIndex, "Round not finished");
        (uint openRoundsIndex, ) = getOpenRound(roundIndex);
        deleteOpenRound(openRoundsIndex);
    }

    function validateScores(
        uint64[] memory scores,
        uint scoresAlreadySubmitted
    ) private pure returns (uint) {
        uint64 sum = 0;
        for (uint i = 0; i < scores.length; i++) {
            sum += scores[i];
        }
        require(sum <= MAX_SCORE, "Sum of scores too big");
        require(
            sum + scoresAlreadySubmitted <= MAX_SCORE,
            "Sum of scores including historic too big"
        );
        return sum;
    }

    function reward(
        address payable[] memory addresses,
        uint64[] memory scores
    ) private {
        for (uint i = 0; i < addresses.length; i++) {
            balances[addresses[i]] += (scores[i] * roundReward) / MAX_SCORE;
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
        if (balances[account] == 0) {
            delete balances[account];
        }
        require(target.send(value), "Withdrawal failed");
        emit Withdrawal(account, target, value);
    }

    function withdraw(address payable target, uint value) public {
        _withdraw(msg.sender, target, value);
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
            abi.encode(account, msg.sender, target, value)
        );
        address signer = getSigner(digest, v, r, s);
        require(signer == account, "Invalid signature");
        // TODO: Add nonce to signature

        require(balances[account] > 0.1 ether, "Insufficient balance");
        balances[account] -= 0.1 ether;
        require(payable(msg.sender).send(0.1 ether), "Gas withdrawal failed");
        _withdraw(account, target, value - 0.1 ether);
    }

    function getSigner(
        bytes32 digest,
        uint8 v,
        bytes32 r,
        bytes32 s
    ) public pure returns (address) {
        bytes memory prefix = "\x19Ethereum Signed Message:\n32";
        bytes32 prefixedDigest = keccak256(abi.encodePacked(prefix, digest));
        return ecrecover(prefixedDigest, v, r, s);
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
