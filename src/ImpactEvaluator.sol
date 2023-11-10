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
    uint public previousRoundRemainingReward;

    uint public nextRoundLength = 10;
    uint public roundReward = 100 ether;

    uint64 public constant MAX_SCORE = 1e15;

    event MeasurementsAdded(
        string cid,
        uint roundIndex,
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
        uint availableInContract = availableBalance() -
            previousRoundRemainingReward -
            currentRoundRoundReward;
        uint nextAvailableRoundReward = availableInContract < roundReward
            ? availableInContract
            : roundReward;
        previousRoundIndex = currentRoundIndex;
        previousRoundTotalScores = 0;
        previousRoundRoundReward = currentRoundRoundReward;
        previousRoundRemainingReward = currentRoundRoundReward;
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

    function addMeasurements(string memory cid) public virtual returns (uint) {
        uint measurementsRoundIndex = currentRoundIndex;
        emit MeasurementsAdded(cid, measurementsRoundIndex, msg.sender);
        tick();
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
            previousRoundIndex != currentRoundIndex &&
                roundIndex == previousRoundIndex,
            "Can only score previous round"
        );

        uint sumOfScores = validateScores(scores);
        reward(addresses, scores);
        // Scores have passed `validateScores()`
        unchecked {
            previousRoundTotalScores += sumOfScores;
        }
    }

    function validateScores(uint64[] memory scores) public view returns (uint) {
        uint64 sum = 0;
        for (uint i = 0; i < scores.length; i++) {
            sum += scores[i];
        }
        require(sum <= MAX_SCORE, "Sum of scores too big");
        require(
            sum + previousRoundTotalScores <= MAX_SCORE,
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
            uint amount;
            // Scores have passed `validateScores()`
            unchecked {
                amount = (scores[i] * previousRoundRoundReward) / MAX_SCORE;
            }
            if (participant != 0x000000000000000000000000000000000000dEaD) {
                increaseParticipantBalance(participant, amount);
            }
            // Scores have passed `validateScores()`
            unchecked {
                previousRoundRemainingReward -= amount;
            }
        }
    }

    function releaseRewards() public onlyAdmin {
        _releaseRewards();
        transferScheduled();
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
