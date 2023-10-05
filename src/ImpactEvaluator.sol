// SPDX-License-Identifier: (MIT or Apache-2.0)

import "../lib/openzeppelin-contracts/contracts/access/AccessControl.sol";
pragma solidity ^0.8.19;

contract ImpactEvaluator is AccessControl {
    struct Round {
        uint end;
        string[] measurementsCids;
        address payable[] addresses;
        uint64[] scores;
        bool scoresSubmitted;
        bool exists;
    }

    Round[] public rounds;
    uint public nextRoundLength = 10;
    uint public roundReward = 100 ether;
    uint public maxStoredRounds = 1000;
    uint64 public constant MAX_SCORE = 1e15;

    event MeasurementsAdded(string cid, uint roundIndex, address sender);
    event RoundStart(uint roundIndex);
    event Transfer(address indexed to, uint256 amount);
    event TransferFailed(address indexed to, uint256 amount);

    bytes32 public constant EVALUATE_ROLE = keccak256("EVALUATE_ROLE");

    constructor(address admin) {
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(EVALUATE_ROLE, admin);
        advanceRound();
    }

    receive() external payable {}

    function advanceRound() private {
        rounds.push();
        Round storage round = rounds[rounds.length - 1];
        round.end = block.number + nextRoundLength;
        round.exists = true;
        emit RoundStart(currentRoundIndex());
        if (rounds.length > maxStoredRounds) {
            delete rounds[rounds.length - maxStoredRounds - 1];
        }
    }

    function adminAdvanceRound() public {
        require(hasRole(DEFAULT_ADMIN_ROLE, msg.sender), "Not an admin");
        advanceRound();
    }

    function maybeAdvanceRound() private {
        uint currentRoundEnd = rounds[currentRoundIndex()].end;
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

    function setMaxStoredRounds(uint _maxStoredRounds) public {
        require(hasRole(DEFAULT_ADMIN_ROLE, msg.sender), "Not an admin");
        require(_maxStoredRounds > 0, "Max stored rounds must be positive");
        if (
            _maxStoredRounds < maxStoredRounds &&
            rounds.length > _maxStoredRounds
        ) {
            for (uint i = 0; i < rounds.length - _maxStoredRounds; i++) {
                delete rounds[i];
            }
        }
        maxStoredRounds = _maxStoredRounds;
    }

    function addMeasurements(string memory cid) public returns (uint) {
        maybeAdvanceRound();
        uint roundIndex = currentRoundIndex();
        rounds[roundIndex].measurementsCids.push(cid);
        emit MeasurementsAdded(cid, roundIndex, msg.sender);
        return roundIndex;
    }

    function setScores(
        uint roundIndex,
        address payable[] memory addresses,
        uint64[] memory scores
    ) public {
        require(hasRole(EVALUATE_ROLE, msg.sender), "Not an evaluator");
        require(
            addresses.length == scores.length,
            "Addresses and scores length mismatch"
        );
        require(roundIndex < currentRoundIndex(), "Round not finished");

        Round storage round = rounds[roundIndex];
        require(round.exists, "Round does not exist");
        require(!round.scoresSubmitted, "Scores already submitted");

        for (uint i = 0; i < addresses.length; i++) {
            round.addresses.push(addresses[i]);
            round.scores.push(scores[i]);
        }

        if (allScoresSubmitted(round)) {
            round.scoresSubmitted = true;
            reward(round.addresses, round.scores);
        }
    }

    function allScoresSubmitted(
        Round storage round
    ) private view returns (bool) {
        uint totalScore = 0;
        for (uint i = 0; i < round.scores.length; i++) {
            totalScore += round.scores[i];
        }
        return totalScore >= MAX_SCORE;
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

    function currentRoundIndex() public view returns (uint) {
        return rounds.length - 1;
    }

    function getRoundEnd(uint index) public view returns (uint) {
        require(rounds[index].exists, "Round does not exist");
        return rounds[index].end;
    }

    function getRoundMeasurementsCids(
        uint index
    ) public view returns (string[] memory) {
        return rounds[index].measurementsCids;
    }

    function getRoundScoresSubmitted(uint index) public view returns (bool) {
        require(rounds[index].exists, "Round does not exist");
        return rounds[index].scoresSubmitted;
    }

    function getParticipantScore(
        uint roundIndex,
        address participant
    ) public view returns (uint) {
        require(rounds[roundIndex].exists, "Round does not exist");
        for (uint i = 0; i < rounds[roundIndex].addresses.length; i++) {
            if (rounds[roundIndex].addresses[i] == payable(participant)) {
                return rounds[roundIndex].scores[i];
            }
        }
        revert("Participant not found");
    }

    function getRoundExists(uint index) public view returns (bool) {
        return rounds[index].exists;
    }

    function currentRoundMeasurementCount() public view returns (uint) {
        return rounds[currentRoundIndex()].measurementsCids.length;
    }
}
