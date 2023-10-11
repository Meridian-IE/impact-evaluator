// SPDX-License-Identifier: (MIT or Apache-2.0)

import "../lib/openzeppelin-contracts/contracts/access/AccessControl.sol";
pragma solidity ^0.8.19;

contract ImpactEvaluator is AccessControl {
    struct Round {
        uint end;
        bool exists;
        uint totalScores;
    }

    mapping(uint => Round) public openRounds;
    uint[] public openRoundIndexes;
    uint public currentRoundIndex;
    uint public nextRoundLength = 10;
    uint public roundReward = 100 ether;
    uint64 public constant MAX_SCORE = 1e15;

    event MeasurementsAdded(
        string cid,
        uint roundIndex,
        address indexed sender
    );
    event RoundStart(uint roundIndex);
    event Transfer(address indexed to, uint256 amount);
    event TransferFailed(address indexed to, uint256 amount);

    bytes32 public constant EVALUATE_ROLE = keccak256("EVALUATE_ROLE");

    constructor(address admin) {
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(EVALUATE_ROLE, admin);
        initializeCurrentRound();
    }

    receive() external payable {}

    function initializeCurrentRound() private {
        Round storage round = openRounds[currentRoundIndex];
        round.end = block.number + nextRoundLength;
        round.exists = true;
        openRoundIndexes.push(currentRoundIndex);
        emit RoundStart(currentRoundIndex);
    }

    function advanceRound() private {
        currentRoundIndex++;
        initializeCurrentRound();
    }

    function adminAdvanceRound() public onlyAdmin {
        advanceRound();
    }

    function maybeAdvanceRound() private {
        uint currentRoundEnd = openRounds[currentRoundIndex].end;
        if (block.number >= currentRoundEnd) {
            advanceRound();
        }
    }

    function setNextRoundLength(uint _nextRoundLength) public onlyAdmin {
        require(_nextRoundLength > 0, "Next round length must be positive");
        nextRoundLength = _nextRoundLength;
    }

    function setRoundReward(uint _roundReward) public onlyAdmin {
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
    ) public onlyEvaluator {
        require(
            addresses.length == scores.length,
            "Addresses and scores length mismatch"
        );
        require(roundIndex < currentRoundIndex, "Round not finished");

        Round storage round = openRounds[roundIndex];
        require(round.exists, "Open round does not exist");

        uint sumOfScores = validateScores(scores, round.totalScores);
        reward(addresses, scores);
        round.totalScores += sumOfScores;

        if (round.totalScores == MAX_SCORE) {
            deleteRound(roundIndex);
        }
    }

    function deleteRound(uint roundIndex) private {
        delete openRounds[roundIndex];

        // Find index inside `openRoundIndexes` and remove it while shrinking
        // the array.
        uint openRoundIndexesIndex;
        for (uint i = 0; i < openRoundIndexes.length; i++) {
            if (openRoundIndexes[i] == roundIndex) {
                openRoundIndexesIndex = i;
                break;
            }
        }
        for (
            uint i = openRoundIndexesIndex;
            i < openRoundIndexes.length - 1;
            i++
        ) {
            openRoundIndexes[i] = openRoundIndexes[i + 1];
        }
        openRoundIndexes.pop();
    }

    function validateScores(
        uint64[] memory scores,
        uint scoresAlreadySubmitted
    ) private pure returns (uint) {
        uint64 sum = 0;
        for (uint i = 0; i < scores.length; i++) {
            sum += scores[i];
        }
        require(sum <= MAX_SCORE, "Sum of scores too big");
        require(
            sum + scoresAlreadySubmitted <= MAX_SCORE,
            "Sum of scores including historic too big"
        );
        return sum;
    }

    function reward(
        address payable[] memory addresses,
        uint64[] memory scores
    ) private {
        uint totalAmount = 0;
        uint[] memory amounts = new uint[](addresses.length);

        for (uint i = 0; i < addresses.length; i++) {
            uint score = scores[i];
            uint256 amount = (score * roundReward) / MAX_SCORE;
            amounts[i] = amount;
            totalAmount += amount;
        }

        require(address(this).balance >= totalAmount, "Not enough funds");

        for (uint i = 0; i < addresses.length; i++) {
            address payable addr = addresses[i];
            uint256 amount = amounts[i];
            if (addr.send(amount)) {
                emit Transfer(addr, amount);
            } else {
                emit TransferFailed(addr, amount);
            }
        }
    }

    function getOpenRoundIndexes() public view returns (uint[] memory) {
        return openRoundIndexes;
    }

    modifier onlyAdmin() {
        require(hasRole(DEFAULT_ADMIN_ROLE, msg.sender), "Not an admin");
        _;
    }

    modifier onlyEvaluator() {
        require(hasRole(EVALUATE_ROLE, msg.sender), "Not an evaluator");
        _;
    }
}
