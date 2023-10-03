// SPDX-License-Identifier: (MIT or Apache-2.0)

import "../lib/openzeppelin-contracts/contracts/access/AccessControl.sol";
pragma solidity ^0.8.19;

contract ImpactEvaluator is AccessControl {
    struct Round {
        uint end;
        bool exists;
    }

    mapping(uint => Round) public openRounds;
    uint public currentRoundIndex;
    uint public nextRoundLength = 10;
    uint public roundReward = 100 ether;

    event MeasurementsAdded(string cid, uint roundIndex, address sender);
    event RoundStart(uint roundIndex);
    event Transfer(address indexed to, uint256 amount);
    event TransferFailed(address indexed to, uint256 amount);

    bytes32 public constant EVALUATE_ROLE = keccak256("EVALUATE_ROLE");

    constructor(address admin) {
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(EVALUATE_ROLE, admin);
        advanceRound(true);
    }

    receive() external payable {}

    function advanceRound(bool firstRound) private {
        if (!firstRound) {
            currentRoundIndex++;
        }
        Round storage round = openRounds[currentRoundIndex];
        round.end = block.number + nextRoundLength;
        round.exists = true;
        emit RoundStart(currentRoundIndex);
    }

    function adminAdvanceRound() public {
        require(hasRole(DEFAULT_ADMIN_ROLE, msg.sender), "Not an admin");
        advanceRound(false);
    }

    function maybeAdvanceRound() private {
        uint currentRoundEnd = openRounds[currentRoundIndex].end;
        if (block.number >= currentRoundEnd) {
            advanceRound(false);
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
        emit MeasurementsAdded(cid, currentRoundIndex, msg.sender);
        return currentRoundIndex;
    }

    function setScores(
        uint roundIndex,
        address payable[] memory addresses,
        uint64[] memory scores
    ) public {
        require(hasRole(EVALUATE_ROLE, msg.sender), "Not an evaluator");
        require(
            addresses.length == scores.length,
            "Addresses and scores length mismatch"
        );
        Round storage round = openRounds[roundIndex];
        require(round.exists, "Open round does not exist");
        reward(addresses, scores);
        delete openRounds[roundIndex];
    }

    function reward(
        address payable[] memory addresses,
        uint64[] memory scores
    ) private {
        require(address(this).balance >= roundReward, "Not enough funds");
        for (uint i = 0; i < addresses.length; i++) {
            address payable addr = addresses[i];
            uint score = scores[i];
            uint256 amount = (score * roundReward) / 1e15;
            if (addr.send(amount)) {
                emit Transfer(addr, amount);
            } else {
                emit TransferFailed(addr, amount);
            }
        }
    }
}
