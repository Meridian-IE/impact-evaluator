// SPDX-License-Identifier: (MIT or Apache-2.0)

import "../lib/openzeppelin-contracts/contracts/access/AccessControl.sol";
pragma solidity ^0.8.19;

contract ImpactEvaluator is AccessControl {
    struct Round {
        uint end;
        string[] measurementCids;
        address payable[] participantAddresses;
        uint[] participantScores;
        bool scoresSubmitted;
        string summaryText;
    }

    Round[] public rounds;
    uint public nextRoundLength = 10;
    uint public roundReward = 100;

    event MeasurementAdded(string cid, uint roundIndex);
    event RoundStart(uint roundIndex);

    bytes32 public constant EVALUATE_ROLE = keccak256("EVALUATE_ROLE");

    constructor(address admin) {
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(EVALUATE_ROLE, admin);
        advanceRound();
    }

    receive() external payable {}

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

    function setRoundReward(uint _roundReward) public {
        require(hasRole(DEFAULT_ADMIN_ROLE, msg.sender), "Not an admin");
        roundReward = _roundReward;
    }

    function addMeasurements(string memory cid) public returns (uint) {
        uint roundIndex = currentRoundIndex();
        rounds[roundIndex].measurementCids.push(cid);
        emit MeasurementAdded(cid, roundIndex);
        maybeAdvanceRound();
        return roundIndex;
    }

    function setScores(
        uint roundIndex,
        address payable[] memory addresses,
        uint[] memory scores,
        string memory summaryText
    ) public {
        require(hasRole(EVALUATE_ROLE, msg.sender), "Not an evaluator");
        require(roundIndex <= rounds.length - 2, "Wrong round");
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

    function reward(
        address payable[] memory addresses,
        uint[] memory scores
    ) private {
        require(address(this).balance >= roundReward, "Not enough funds");
        for (uint i = 0; i < addresses.length; i++) {
            address payable addr = addresses[i];
            uint score = scores[i];
            addr.transfer((score / 1000000000000000) * roundReward);
        }
    }

    function currentRoundIndex() public view returns (uint) {
        return rounds.length - 1;
    }

    function getRound(uint index) public view returns (Round memory) {
        return rounds[index];
    }

    function currentRoundMeasurementCount() public view returns (uint) {
        return rounds[currentRoundIndex()].measurementCids.length;
    }
}
