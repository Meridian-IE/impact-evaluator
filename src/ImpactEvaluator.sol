// SPDX-License-Identifier: (MIT or Apache-2.0)

import "../lib/openzeppelin-contracts/contracts/access/AccessControl.sol";
pragma solidity ^0.8.19;

contract ImpactEvaluator is AccessControl {
    struct Round {
        uint end;
        string[] measurementCids;
        address[] participantAddresses;
        uint[] participantScores;
        bool scoresSubmitted;
        string summaryText;
    }

    Round[] public rounds;
    uint nextRoundLength;

    event MeasurementAdded(string cid, uint roundIndex);
    event RoundStart(uint roundIndex);
    
    bytes32 public constant EVALUATE_ROLE = keccak256("EVALUATE_ROLE");

    constructor(address admin, uint roundLength) {
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(EVALUATE_ROLE, admin);
        nextRoundLength = roundLength;
        advanceRound();
    }

    function advanceRound() private {
        Round memory round;
        round.end = block.number + nextRoundLength;
        rounds.push(round);
        emit RoundStart(currentRoundIndex());
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
        nextRoundLength = _nextRoundLength;
    }

    function addMeasurement(string memory cid) public {
        rounds[currentRoundIndex()].measurementCids.push(cid);
        emit MeasurementAdded(cid, currentRoundIndex());
        maybeAdvanceRound();
    }

    function setScores(
        uint roundIndex,
        address[] memory addresses,
        uint[] memory scores,
        string memory summaryText
    ) public {
        require(hasRole(EVALUATE_ROLE, msg.sender), "Not an evaluator");
        require(roundIndex == rounds.length - 2, "Wrong round");
        require(
            addresses.length == scores.length,
            "Addresses and scores length mismatch"
        );
        Round memory round = rounds[roundIndex];
        require(!round.scoresSubmitted, "Scores already submitted");
        round.participantAddresses = addresses;
        round.participantScores = scores;
        round.summaryText = summaryText;
        round.scoresSubmitted = true;
        rounds[roundIndex] = round;
        reward(addresses, scores);
    }

    function reward(address[] memory addresses, uint[] memory _scores) private {
        // PaymentsFactory.deploy(reserve, scores);
    }

    function currentRoundIndex() public view returns (uint) {
        return rounds.length - 1;
    }

    function getRound(uint index) public view returns (Round memory) {
        return rounds[index];
    }
}
