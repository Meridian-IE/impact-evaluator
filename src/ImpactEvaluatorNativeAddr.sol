// SPDX-License-Identifier: (MIT or Apache-2.0)

// Keep up to date with ImpactEvaluator.sol!

import "../lib/openzeppelin-contracts/contracts/access/AccessControl.sol";
import "../lib/filecoin-solidity/contracts/v0.8/utils/FilAddresses.sol";
import "../lib/filecoin-solidity/contracts/v0.8/SendAPI.sol";
pragma solidity ^0.8.19;

contract ImpactEvaluatorNativeAddr is AccessControl {
    using SendAPI for CommonTypes.FilAddress;

    struct Round {
        uint end;
        string[] measurementsCids;
        CommonTypes.FilAddress[] participants;
        uint64[] scores;
        bool scoresSubmitted;
        string summaryText;
        bool exists;
    }

    Round[] public rounds;
    uint public nextRoundLength = 10;
    uint public roundReward = 100;
    uint public maxStoredRounds = 1000;

    event MeasurementsAdded(string cid, uint roundIndex, address sender);
    event RoundStart(uint roundIndex);
    event Transfer(CommonTypes.FilAddress indexed to, uint256 amount);
    event TransferFailed(CommonTypes.FilAddress indexed to, uint256 amount);

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
        if (_maxStoredRounds < maxStoredRounds) {
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
        CommonTypes.FilAddress[] memory addresses,
        uint64[] memory scores,
        string memory summaryText
    ) public {
        require(hasRole(EVALUATE_ROLE, msg.sender), "Not an evaluator");
        require(
            addresses.length == scores.length,
            "Addresses and scores length mismatch"
        );
        Round storage round = rounds[roundIndex];
        require(round.exists, "Round does not exist");
        require(!round.scoresSubmitted, "Scores already submitted");
        for (uint i = 0; i < addresses.length; i++) {
          round.participants.push(addresses[i]);
        }
        round.scores = scores;
        round.summaryText = summaryText;
        round.scoresSubmitted = true;
        reward(addresses, scores);
    }

    function reward(
        CommonTypes.FilAddress[] memory addresses,
        uint64[] memory scores
    ) private {
        require(address(this).balance >= roundReward, "Not enough funds");
        for (uint i = 0; i < addresses.length; i++) {
            CommonTypes.FilAddress memory addr = addresses[i];
            uint score = scores[i];
            uint256 amount = (score / 1000000000000000) * roundReward;
            addr.send(amount);
            // if (success) {
            //     emit Transfer(addr, amount);
            // } else {
            //     emit TransferFailed(addr, amount);
            // }
        }
    }

    function currentRoundIndex() public view returns (uint) {
        return rounds.length - 1;
    }

    function getRoundEnd(uint index) public view returns (uint) {
        return rounds[index].end;
    }

    function getRoundMeasurementsCids(uint index) public view returns (string[] memory) {
        return rounds[index].measurementsCids;
    }

    function getRoundSummaryText(uint index) public view returns (string memory) {
        return rounds[index].summaryText;
    }

    function getRoundScoresSubmitted(uint index) public view returns (bool) {
        return rounds[index].scoresSubmitted;
    }

    function getScores(uint roundIndex) public view returns (uint64[] memory) {
      return rounds[roundIndex].scores;
    }

    function getRoundExists(uint index) public view returns (bool) {
        return rounds[index].exists;
    }

    function currentRoundMeasurementCount() public view returns (uint) {
        return rounds[currentRoundIndex()].measurementsCids.length;
    }
}
