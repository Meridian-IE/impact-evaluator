// SPDX-License-Identifier: (MIT or Apache-2.0)

import "../lib/openzeppelin-contracts/contracts/access/AccessControl.sol";
pragma solidity ^0.8.19;

contract ImpactEvaluator is AccessControl {
    struct Round {
        uint index;
        uint end;
        string[] measurementsCids;
        address payable[] participantAddresses;
        uint[] participantScores;
        bool scoresSubmitted;
        string summaryText;
        bool exists;
    }
    // Recursive structs can't be returned from public methods, therefore have
    // a separate type to implement the linked list, and return `Round` to the
    // public
    struct LinkedRound {
        Round round;
        // Recursive structs aren't possible, use a single element array instead
        LinkedRound[] previousLinkedRound;
    }

    LinkedRound public currentLinkedRound;
    LinkedRound public oldestStoredLinkedRound;
    uint public nextRoundLength = 10;
    uint public roundReward = 100;
    uint public maxStoredRounds = 1000;

    event MeasurementsAdded(string cid, uint roundIndex);
    event RoundStart(uint roundIndex);

    bytes32 public constant EVALUATE_ROLE = keccak256("EVALUATE_ROLE");

    constructor(address admin) {
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(EVALUATE_ROLE, admin);
        advanceRound();
    }

    receive() external payable {}

    function advanceRound() private {
        LinkedRound memory linkedRound;
        if (currentLinkedRound.round.exists) {
            linkedRound.round.index = currentLinkedRound.round.index + 1;
            linkedRound.previousLinkedRound[0] = currentLinkedRound;
        } else {
            linkedRound.round.index = 0;
        }
        linkedRound.round.end = block.number + nextRoundLength;
        linkedRound.round.exists = true;
        currentLinkedRound = linkedRound;
        emit RoundStart(linkedRound.round.index);
        maybeRemoveOldestLinkedRound();
    }

    function maybeRemoveOldestLinkedRound() private {
        if (currentLinkedRound.round.index >= maxStoredRounds) {
            LinkedRound memory linkedRound = getLinkedRound(
                currentLinkedRound.round.index - maxStoredRounds
            );
            delete linkedRound.previousLinkedRound[0];
        }
    }

    function adminAdvanceRound() public {
        require(hasRole(DEFAULT_ADMIN_ROLE, msg.sender), "Not an admin");
        advanceRound();
    }

    function maybeAdvanceRound() private {
        if (block.number >= currentLinkedRound.round.end) {
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

    function setMaxStoredRounds(uint _maxStoredRounds) public {
        require(hasRole(DEFAULT_ADMIN_ROLE, msg.sender), "Not an admin");
        maxStoredRounds = _maxStoredRounds;
    }

    function addMeasurements(string memory cid) public returns (uint) {
        currentLinkedRound.round.measurementsCids.push(cid);
        emit MeasurementsAdded(cid, currentLinkedRound.round.index);
        maybeAdvanceRound();
        return currentLinkedRound.round.index;
    }

    function setScores(
        uint roundIndex,
        address payable[] memory addresses,
        uint[] memory scores,
        string memory summaryText
    ) public {
        require(hasRole(EVALUATE_ROLE, msg.sender), "Not an evaluator");
        require(roundIndex <= currentRoundIndex() - 2, "Wrong round");
        require(
            addresses.length == scores.length,
            "Addresses and scores length mismatch"
        );
        LinkedRound memory linkedRound = getLinkedRound(roundIndex);
        require(!linkedRound.round.scoresSubmitted, "Scores already submitted");
        linkedRound.round.participantAddresses = addresses;
        linkedRound.round.participantScores = scores;
        linkedRound.round.summaryText = summaryText;
        linkedRound.round.scoresSubmitted = true;
        getLinkedRound(roundIndex + 1).previousLinkedRound[0] = linkedRound;
        if (scores.length > 0) {
            reward(addresses, scores);
        }
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
        return currentLinkedRound.round.index;
    }

    // TODO: Recursive return type not allowed. Wrap round in LinkedRound
    function getRound(uint index) public view returns (Round memory) {
        return getLinkedRound(index).round;
    }

    function getLinkedRound(uint index) private view returns (LinkedRound memory) {
        LinkedRound memory linkedRound = currentLinkedRound;
        while (linkedRound.round.index != index) {
            linkedRound = linkedRound.previousLinkedRound[0];
            require(linkedRound.round.exists, "Round does not exist (any more)");
        }
        return linkedRound;
    }

    function currentRoundMeasurementCount() public view returns (uint) {
        return currentLinkedRound.round.measurementsCids.length;
    }
}
