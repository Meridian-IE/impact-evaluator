// SPDX-License-Identifier: (MIT or Apache-2.0)

import "../lib/openzeppelin-contracts/contracts/access/AccessControl.sol";
pragma solidity ^0.8.19;

struct Round {
    string[] measurementCids;
    string[] measurementProviders;
    address[] participantAddresses;
    uint[] participantScores;
    bool scoresSubmitted;
}

/**
 * @title Impact Evaluator Contract
 * @dev This contract evaluates impact of peers in a system.
 */
contract ImpactEvaluator is AccessControl {
    Round[] public rounds;
    address[] public evaluators;

    event MeasurementAdded(string cid, string provider);
    event RoundStart(uint roundIndex);
    
    bytes32 public constant COMMITMENT_ROLE = keccak256("COMMITMENT_ROLE");
    bytes32 public constant EVALUATE_ROLE = keccak256("EVALUATE_ROLE");

    /**
     * @dev Creates an impact evaluator contract
     * @param admin The address of the factory admin
     * @param _evaluators The addresses of the evaluators
     **/
    constructor(address admin, address[] memory _evaluators) {
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(COMMITMENT_ROLE, admin);
        _grantRole(EVALUATE_ROLE, admin);
        evaluators = _evaluators;
        advanceRound();
    }

    function advanceRound() private {
        Round memory round;
        rounds.push(round);
        emit RoundStart(rounds.length - 1);
    }

    function maybeAdvanceRound() private {
        // TODO: Define round advance logic. Base on tipset?
        bool advance = false;
        if (advance) {
            advanceRound();
        }
    }

    /**
     * @dev Adds a measurement.
     * @param cid CID of measurements submitted
     * @param provider Where to retrieve the CID from
     **/
    function addMeasurement(string memory cid, string memory provider) public {
        // require(hasRole(COMMITMENT_ROLE, msg.sender));
        rounds[rounds.length - 1].measurementCids.push(cid);
        rounds[rounds.length - 1].measurementProviders.push(provider);
        emit MeasurementAdded(cid, provider);
        maybeAdvanceRound();
    }

    function setScores(uint roundIndex, address[] memory addresses, uint[] memory scores) public {
        require(hasRole(EVALUATE_ROLE, msg.sender));
        require(roundIndex == rounds.length - 2);
        require(addresses.length == scores.length);
        Round memory round = rounds[roundIndex];
        require(!round.scoresSubmitted);
        round.participantAddresses = addresses;
        round.participantScores = scores;
        round.scoresSubmitted = true;
        rounds[roundIndex] = round;
        reward(addresses, scores);
    }

    function reward(address[] memory addresses, uint[] memory _scores) private {
        // PaymentsFactory.deploy(reserve, scores);
    }
}
