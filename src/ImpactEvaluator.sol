// SPDX-License-Identifier: (MIT or Apache-2.0)

import "../lib/openzeppelin-contracts/contracts/access/AccessControl.sol";
import "./Balances.sol";
pragma solidity ^0.8.19;

contract ImpactEvaluator is AccessControl, Balances {
    uint public currentRoundIndex;
    uint public currentRoundEndBlockNumber;
    uint public currentRoundRoundReward;

    uint public previousRoundIndex;
    uint public previousRoundTotalScores;
    uint public previousRoundRoundReward;

    uint public nextRoundLength = 10;
    uint public roundReward = 100 ether;
    uint public balanceHeld = 0;

    uint public constant MAX_SCORE = 1e15;

    event MeasurementsAdded(
        string cid,
        uint indexed roundIndex,
        address indexed sender
    );
    event RoundStart(uint roundIndex);

    bytes32 public constant EVALUATE_ROLE = keccak256("EVALUATE_ROLE");

    constructor(address admin) {
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(EVALUATE_ROLE, admin);
        advanceRound();
    }

    receive() external payable {}

    function advanceRound() private {
        uint previousRoundRemainingReward = (1 -
            (previousRoundTotalScores / MAX_SCORE)) * previousRoundRoundReward;
        uint availableInContract = address(this).balance -
            balanceHeld -
            previousRoundRemainingReward -
            currentRoundRoundReward;
        uint nextAvailableRoundReward = availableInContract < roundReward
            ? availableInContract
            : roundReward;
        previousRoundIndex = currentRoundIndex;
        previousRoundTotalScores = 0;
        previousRoundRoundReward = currentRoundRoundReward;
        currentRoundIndex = currentRoundEndBlockNumber == 0
            ? 0
            : currentRoundIndex + 1;
        currentRoundEndBlockNumber = block.number + nextRoundLength;
        currentRoundRoundReward = nextAvailableRoundReward;
        emit RoundStart(currentRoundIndex);
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

    function setMaxTransfersPerTx(uint _maxTransfersPerTx) public onlyAdmin {
        _setMaxTransfersPerTx(_maxTransfersPerTx);
    }

    function tick() public {
        if (block.number >= currentRoundEndBlockNumber) {
            advanceRound();
        }
        transferScheduled();
    }

    function addMeasurements(
        string calldata cid
    ) public virtual returns (uint) {
        uint measurementsRoundIndex = currentRoundIndex;
        emit MeasurementsAdded(cid, measurementsRoundIndex, msg.sender);
        tick();
        return measurementsRoundIndex;
    }

    function setScores(
        uint roundIndex,
        address payable[] calldata addresses,
        uint[] calldata scores
    ) public onlyEvaluator {
        require(
            addresses.length == scores.length,
            "Addresses and scores length mismatch"
        );
        require(
            previousRoundIndex != currentRoundIndex &&
                roundIndex == previousRoundIndex,
            "Can only score previous round"
        );

        uint sumOfScores = 0;
        uint addedBalance = 0;
        for (uint i = 0; i < addresses.length; i++) {
            uint score = scores[i];
            address payable participant = addresses[i];
            sumOfScores += score;
            uint amount = (score * previousRoundRoundReward) / MAX_SCORE;
            if (participant != 0x000000000000000000000000000000000000dEaD) {
                increaseParticipantBalance(participant, amount);
                addedBalance += amount;
            }
        }
        require(
            sumOfScores + previousRoundTotalScores <= MAX_SCORE,
            "Sum of scores including historic too big"
        );
        previousRoundTotalScores += sumOfScores;
        balanceHeld += addedBalance;
    }

    function releaseRewards() public onlyAdmin {
        _releaseRewards();
        balanceHeld -= transferScheduled();
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
