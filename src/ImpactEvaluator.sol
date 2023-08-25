// SPDX-License-Identifier: (MIT or Apache-2.0)

import "../lib/openzeppelin-contracts/contracts/access/AccessControl.sol";
pragma solidity ^0.8.19;

contract ImpactEvaluator is AccessControl {
    struct Round {
        uint start;
        string[] measurementCids;
        address payable[] participantAddresses;
        uint[] participantScores;
        bool scoresSubmitted;
        string summaryText;
    }

    Round[] public rounds;
    uint public roundLength;
    uint public roundReward;

    event MeasurementAdded(string cid, uint roundIndex);
    event RoundStart(uint roundIndex);

    bytes32 public constant EVALUATE_ROLE = keccak256("EVALUATE_ROLE");

    constructor(address admin, uint _roundLength, uint _roundReward) {
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(EVALUATE_ROLE, admin);
        roundLength = _roundLength;
        roundReward = _roundReward;
        advanceRound();
    }

    receive() external payable {}

    function advanceRound() private {
        Round memory round;
        round.start = block.number;
        rounds.push(round);
        emit RoundStart(currentRoundIndex());
    }

    function adminAdvanceRound() public {
        require(hasRole(DEFAULT_ADMIN_ROLE, msg.sender), "Not an admin");
        advanceRound();
    }

    function maybeAdvanceRound() private {
        uint currentRoundStart = rounds[currentRoundIndex()].start;
        if (block.number - currentRoundStart >= roundLength) {
            advanceRound();
        }
    }

    function setRoundReward(uint _roundReward) public {
        require(hasRole(DEFAULT_ADMIN_ROLE, msg.sender), "Not an admin");
        roundReward = _roundReward;
    }

    function addMeasurement(string memory cid) public {
        rounds[currentRoundIndex()].measurementCids.push(cid);
        emit MeasurementAdded(cid, currentRoundIndex());
        maybeAdvanceRound();
    }

    function setScores(
        uint roundIndex,
        address payable[] memory addresses,
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

    function reward(
        address payable[] memory addresses,
        uint[] memory scores
    ) private {
        require(address(this).balance >= roundReward, "Not enough funds");
        for (uint i = 0; i < addresses.length; i++) {
            address payable addr = addresses[i];
            uint score = scores[i];
            addr.transfer((score / 1000000) * roundReward);
        }
    }

    function currentRoundIndex() public view returns (uint) {
        return rounds.length - 1;
    }

    function getRound(uint index) public view returns (Round memory) {
        return rounds[index];
    }
}
