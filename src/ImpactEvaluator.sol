// SPDX-License-Identifier: (MIT or Apache-2.0)

import "../lib/openzeppelin-contracts/contracts/access/AccessControl.sol";
pragma solidity ^0.8.19;

contract ImpactEvaluator is AccessControl {
    struct Round {
        uint end;
        string[] measurementsCids;
        address payable[] participants;
        uint[] scores;
        bool scoresSubmitted;
        string summaryText;
    }

    Round[] public rounds;
    uint public nextRoundLength = 10;
    uint public roundReward = 100;

    event MeasurementsAdded(string cid, uint roundIndex);
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
        maybeAdvanceRound();
        uint roundIndex = currentRoundIndex();
        rounds[roundIndex].measurementsCids.push(cid);
        emit MeasurementsAdded(cid, roundIndex);
        return roundIndex;
    }

    function setScores(
        uint roundIndex,
        address payable[] memory addresses,
        uint[] memory scores,
        string memory summaryText
    ) public {
        require(hasRole(EVALUATE_ROLE, msg.sender), "Not an evaluator");
        require(
            addresses.length == scores.length,
            "Addresses and scores length mismatch"
        );
        Round memory round = rounds[roundIndex];
        require(!round.scoresSubmitted, "Scores already submitted");
        round.participants = addresses;
        round.scores = scores;
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
            uint256 amount = (score / 1000000000000000) * roundReward;
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

    function getRound(uint index) public view returns (Round memory) {
        return rounds[index];
    }

    function currentRoundMeasurementCount() public view returns (uint) {
        return rounds[currentRoundIndex()].measurementsCids.length;
    }
}
