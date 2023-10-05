// SPDX-License-Identifier: (MIT or Apache-2.0)

import "../lib/openzeppelin-contracts/contracts/access/AccessControl.sol";
pragma solidity ^0.8.19;

contract ImpactEvaluator is AccessControl {
    struct Round {
        uint end;
        address payable[] addresses;
        uint64[] scores;
        bool exists;
    }

    mapping(uint => Round) public openRounds;
    uint public currentRoundIndex;
    uint public nextRoundLength = 10;
    uint public roundReward = 100 ether;
    uint64 public constant MAX_SCORE = 1e15;

    event MeasurementsAdded(string cid, uint roundIndex, address sender);
    event RoundStart(uint roundIndex);
    event Transfer(address indexed to, uint256 amount);
    event TransferFailed(address indexed to, uint256 amount);

    bytes32 public constant EVALUATE_ROLE = keccak256("EVALUATE_ROLE");

    constructor(address admin) {
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(EVALUATE_ROLE, admin);
        initializeCurrentRound();
    }

    receive() external payable {}

    function initializeCurrentRound() private {
        Round storage round = openRounds[currentRoundIndex];
        round.end = block.number + nextRoundLength;
        round.exists = true;
        emit RoundStart(currentRoundIndex);
    }

    function advanceRound() private {
        currentRoundIndex++;
        initializeCurrentRound();
    }

    function adminAdvanceRound() public {
        require(hasRole(DEFAULT_ADMIN_ROLE, msg.sender), "Not an admin");
        advanceRound();
    }

    function maybeAdvanceRound() private {
        uint currentRoundEnd = openRounds[currentRoundIndex].end;
        if (block.number >= currentRoundEnd) {
            advanceRound();
        }
    }

    function setNextRoundLength(uint _nextRoundLength) public {
        require(hasRole(DEFAULT_ADMIN_ROLE, msg.sender), "Not an admin");
        require(_nextRoundLength > 0, "Next round length must be positive");
        nextRoundLength = _nextRoundLength;
    }

    function setRoundReward(uint _roundReward) public {
        require(hasRole(DEFAULT_ADMIN_ROLE, msg.sender), "Not an admin");
        roundReward = _roundReward;
    }

    function addMeasurements(string memory cid) public returns (uint) {
        maybeAdvanceRound();
        emit MeasurementsAdded(cid, currentRoundIndex, msg.sender);
        return currentRoundIndex;
    }

    function setScores(
        uint roundIndex,
        address payable[] memory addresses,
        uint64[] memory scores,
        bool moreScoresExpected
    ) public {
        require(hasRole(EVALUATE_ROLE, msg.sender), "Not an evaluator");
        require(
            addresses.length == scores.length,
            "Addresses and scores length mismatch"
        );
        require(roundIndex < currentRoundIndex, "Round not finished");

        Round storage round = openRounds[roundIndex];
        require(round.exists, "Open round does not exist");

        if (moreScoresExpected) {
            for (uint i = 0; i < addresses.length; i++) {
                round.addresses.push(addresses[i]);
                round.scores.push(scores[i]);
            }
        } else {
            (
                address payable[] memory mergedAddresses,
                uint64[] memory mergedScores
            ) = mergeScores(round, addresses, scores);
            reward(mergedAddresses, mergedScores);
            cleanUpRound(round, roundIndex);
        }
    }

    function mergeScores(
        Round storage round,
        address payable[] memory addresses,
        uint64[] memory scores
    ) private view returns (address payable[] memory, uint64[] memory) {
        uint mergedLength = round.addresses.length + addresses.length;
        address payable[] memory mergedAddresses = new address payable[](
            mergedLength
        );
        uint64[] memory mergedScores = new uint64[](mergedLength);
        for (uint i = 0; i < round.addresses.length; i++) {
            mergedAddresses[i] = round.addresses[i];
            mergedScores[i] = round.scores[i];
        }
        for (uint i = 0; i < addresses.length; i++) {
            mergedAddresses[round.addresses.length + i] = addresses[i];
            mergedScores[round.addresses.length + i] = scores[i];
        }
        return (mergedAddresses, mergedScores);
    }

    function cleanUpRound(Round storage round, uint roundIndex) private {
        round.end = 0;
        delete round.addresses;
        delete round.scores;
        round.exists = false;
        delete openRounds[roundIndex];
    }

    function reward(
        address payable[] memory addresses,
        uint64[] memory scores
    ) private {
        validateScores(scores);
        require(address(this).balance >= roundReward, "Not enough funds");
        for (uint i = 0; i < addresses.length; i++) {
            address payable addr = addresses[i];
            uint score = scores[i];
            uint256 amount = (score * roundReward) / MAX_SCORE;
            if (addr.send(amount)) {
                emit Transfer(addr, amount);
            } else {
                emit TransferFailed(addr, amount);
            }
        }
    }

    function validateScores(uint64[] memory scores) private pure {
        uint64 sum = 0;
        for (uint i = 0; i < scores.length; i++) {
            sum += scores[i];
        }
        require(sum <= MAX_SCORE, "Sum of scores too big");
    }
}
