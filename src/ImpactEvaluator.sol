// SPDX-License-Identifier: (MIT or Apache-2.0)

import "../lib/openzeppelin-contracts/contracts/access/AccessControl.sol";
pragma solidity ^0.8.19;

/**
 * @title Impact Evaluator Contract
 * @dev This contract evaluates impact of peers in a system.
 */
contract ImpactEvaluator is AccessControl {
    struct Measurement {
        string cid;
        string provider;
    }

    struct Round {
        Measurement[] measurements;
        mapping(address => int) scores;
        bool scoresSubmitted;
    }

    Round[] public rounds;
    address[] public evaluators;

    event MeasurementAdded(Measurement measurement);
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
        maybeAdvanceRound();
    }

    function maybeAdvanceRound() private {
        // TODO: Define round advance logic. Base on tipset?
        bool advance = false;
        if (advance || rounds.length == 0) {
            Round storage round = rounds[rounds.length];
            emit RoundStart(rounds.length - 1);
        }
    }

    /**
     * @dev Adds a measurement.
     * @param cid CID of measurements submitted
     * @param provider Where to retrieve the CID from
     **/
    function addMeasurement(string memory cid, string memory provider) public {
        // require(hasRole(COMMITMENT_ROLE, msg.sender));
        Measurement memory measurement = Measurement(cid, provider);
        rounds[rounds.length - 1].measurements.push(measurement);
        emit MeasurementAdded(measurement);
        maybeAdvanceRound();
    }

    // TODO: Types containing (nested) mappings can only be parameters or return 
    // variables of internal or library functions.
    function setScores(uint roundIndex, mapping(address => uint) memory scores) public {
        require(hasRole(EVALUATE_ROLE, msg.sender));
        require(roundIndex == rounds.length - 2);
        Round storage round = rounds[rounds.length - 2];
        require(!round.scoresSubmitted);
        round.scores = scores;
        round.scoresSubmitted = true;
        reward(scores);
    }

    function reward(mapping(address => uint) memory scores) private {
        // PaymentsFactory.deploy(reserve, scores);
    }
}
