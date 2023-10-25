// SPDX-License-Identifier: (MIT or Apache-2.0)

import "../lib/openzeppelin-contracts/contracts/access/AccessControl.sol";
pragma solidity ^0.8.19;

contract ImpactEvaluator is AccessControl {
    struct Round {
        uint index;
        uint end;
        uint totalScores;
    }

    Round[] public openRounds;
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
        advanceRound();
    }

    receive() external payable {}

    function advanceRound() private {
        uint nextRoundIndex = openRounds.length == 0
            ? 0
            : currentRoundIndex() + 1;
        Round memory round = Round(
            nextRoundIndex,
            block.number + nextRoundLength,
            0
        );
        openRounds.push(round);
        emit RoundStart(currentRoundIndex());
    }

    function adminAdvanceRound() public onlyAdmin {
        advanceRound();
    }

    function maybeAdvanceRound() private {
        uint currentRoundEnd = openRounds[currentRoundIndex()].end;
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

    function addMeasurements(string memory cid) public virtual returns (uint) {
        maybeAdvanceRound();
        emit MeasurementsAdded(cid, currentRoundIndex(), msg.sender);
        return currentRoundIndex();
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
        require(roundIndex < currentRoundIndex(), "Round not finished");

        Round storage round = getOpenRound(roundIndex);
        uint sumOfScores = validateScores(scores, round.totalScores);
        reward(addresses, scores);
        round.totalScores += sumOfScores;

        if (round.totalScores == MAX_SCORE) {
            deleteOpenRound(roundIndex);
        }
    }

    function getOpenRound(
        uint roundIndex
    ) private view returns (Round storage) {
        for (uint i = 0; i < openRounds.length; i++) {
            if (openRounds[i].index == roundIndex) {
                return openRounds[i];
            }
        }
        revert("Open round does not exist");
    }

    function deleteOpenRound(uint roundIndex) private {
        // Find index inside `openRounds` and remove it while shrinking
        // the array.
        uint openRoundsIndex;
        for (uint i = 0; i < openRounds.length; i++) {
            if (openRounds[i].index == roundIndex) {
                openRoundsIndex = i;
                break;
            }
        }
        for (uint i = openRoundsIndex; i < openRounds.length - 1; i++) {
            openRounds[i] = openRounds[i + 1];
        }
        openRounds.pop();
    }

    function adminDeleteOpenRound(uint roundIndex) public onlyAdmin {
        require(roundIndex < currentRoundIndex(), "Round not finished");
        deleteOpenRound(roundIndex);
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
            if (addr == 0x000000000000000000000000000000000000dEaD) {
                // TODO: Shall we emit an event here too?
                continue;
            }
            uint256 amount = amounts[i];
            if (addr.send(amount)) {
                emit Transfer(addr, amount);
            } else {
                emit TransferFailed(addr, amount);
            }
        }
    }

    function currentRoundIndex() public view returns (uint) {
        return openRounds[openRounds.length - 1].index;
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
