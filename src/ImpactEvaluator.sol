// SPDX-License-Identifier: (MIT or Apache-2.0)

import "../lib/openzeppelin-contracts/contracts/access/AccessControl.sol";
pragma solidity ^0.8.19;

contract ImpactEvaluator is AccessControl {
    struct Round {
        uint end;
        string[] measurementsCids;
        address payable[] addresses;
        mapping(address => uint64) scores;
        mapping(uint => bool) scoresSubmitted;
        string summaryText;
        bool exists;
    }

    Round[] public rounds;
    uint public nextRoundLength = 10;
    uint public roundReward = 100 ether;
    uint public maxStoredRounds = 1000;

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
        nextRoundLength = _nextRoundLength;
    }

    function setRoundReward(uint _roundReward) public {
        require(hasRole(DEFAULT_ADMIN_ROLE, msg.sender), "Not an admin");
        roundReward = _roundReward;
    }

    function setMaxStoredRounds(uint _maxStoredRounds) public {
        require(hasRole(DEFAULT_ADMIN_ROLE, msg.sender), "Not an admin");
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
        uint64[] memory scores,
        uint callNumber,
        uint totalCalls
    ) public {
        require(hasRole(EVALUATE_ROLE, msg.sender), "Not an evaluator");
        require(
            addresses.length == scores.length,
            "Addresses and scores length mismatch"
        );
        require(callNumber < totalCalls, "Call number exceeded");

        Round storage round = rounds[roundIndex];
        require(round.exists, "Round does not exist");
        require(
            round.scoresSubmitted[callNumber] == false,
            "Call number already used"
        );
        for (uint i = 0; i < addresses.length; i++) {
            require(round.scores[addresses[i]] == 0, "Address already scored");
            round.addresses.push(addresses[i]);
            round.scores[addresses[i]] = scores[i];
        }
        round.scoresSubmitted[callNumber] = true;
        if (callNumber == totalCalls - 1) {
            reward(round.addresses, round.scores);
        }
    }

    function reward(
        address payable[] memory addresses,
        mapping(address => uint64) storage scores
    ) private {
        require(address(this).balance >= roundReward, "Not enough funds");
        for (uint i = 0; i < addresses.length; i++) {
            address payable addr = addresses[i];
            uint score = scores[addr];
            uint256 amount = (score * roundReward) / 1e15;
            if (addr.send(amount)) {
                emit Transfer(addr, amount);
            } else {
                emit TransferFailed(addr, amount);
            }
        }
    }

    function currentRoundIndex() public view returns (uint) {
        return rounds.length - 1;
    }

    function getRoundEnd(uint index) public view returns (uint) {
        return rounds[index].end;
    }

    function getRoundMeasurementsCids(
        uint index
    ) public view returns (string[] memory) {
        return rounds[index].measurementsCids;
    }

    function getParticipantScore(
        uint roundIndex,
        address participant
    ) public view returns (uint) {
        return rounds[roundIndex].scores[participant];
    }

    function getRoundExists(uint index) public view returns (bool) {
        return rounds[index].exists;
    }

    function currentRoundMeasurementCount() public view returns (uint) {
        return rounds[currentRoundIndex()].measurementsCids.length;
    }
}
