// SPDX-License-Identifier: (MIT or Apache-2.0)

import "../lib/openzeppelin-contracts/contracts/access/AccessControl.sol";
pragma solidity ^0.8.19;



contract ImpactEvaluator is AccessControl {
    struct Round {
        string[] measurementCids;
        string[] measurementProviders;
        address[] participantAddresses;
        uint[] participantScores;
        bool scoresSubmitted;
    }

    Round[] public rounds;

    event MeasurementAdded(string cid, string provider);
    event RoundStart(uint roundIndex);
    
    bytes32 public constant EVALUATE_ROLE = keccak256("EVALUATE_ROLE");

    constructor(address admin) {
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(EVALUATE_ROLE, admin);
        advanceRound();
    }

    function advanceRound() private {
        Round memory round;
        rounds.push(round);
        emit RoundStart(rounds.length - 1);
    }

    function adminAdvanceRound() public {
        require(hasRole(DEFAULT_ADMIN_ROLE, msg.sender), "Not an admin");
        advanceRound();
    }

    function maybeAdvanceRound() private {
        // TODO: Define round advance logic. Base on tipset?
        bool advance = false;
        if (advance) {
            advanceRound();
        }
    }

    function addMeasurement(string memory cid, string memory provider) public {
        rounds[rounds.length - 1].measurementCids.push(cid);
        rounds[rounds.length - 1].measurementProviders.push(provider);
        emit MeasurementAdded(cid, provider);
        maybeAdvanceRound();
    }

    function setScores(uint roundIndex, address[] memory addresses, uint[] memory scores) public {
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
