// SPDX-License-Identifier: (MIT or Apache-2.0)

import "../lib/openzeppelin-contracts/contracts/access/AccessControl.sol";
import "./Balances.sol";
pragma solidity ^0.8.19;

contract ImpactEvaluator is AccessControl, Balances {
    uint public currentRoundIndex;
    uint public currentRoundEndBlockNumber;
    uint public currentRoundRoundReward;

    uint public previousRoundIndex;
    uint public previousRoundTotalScores;
    uint public previousRoundRoundReward;

    uint public nextRoundLength = 10;
    uint public roundReward = 100 ether;
    uint public balanceHeld = 0;

    uint public constant MAX_SCORE = 1e15;

    event MeasurementsAdded(
        string cid,
        uint indexed roundIndex,
        address indexed sender
    );
    event Scores(
        uint indexed roundIndex,
        string cid
    );
    event RoundStart(uint roundIndex);

    bytes32 public constant EVALUATE_ROLE = keccak256("EVALUATE_ROLE");

    constructor(address admin) {
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(EVALUATE_ROLE, admin);
        advanceRound();
    }

    receive() external payable {}

    function advanceRound() private {
        uint previousRoundRemainingReward = (previousRoundRoundReward *
            (MAX_SCORE - previousRoundTotalScores)) / MAX_SCORE;
        uint availableInContract = address(this).balance -
            balanceHeld -
            previousRoundRemainingReward -
            currentRoundRoundReward;
        uint nextAvailableRoundReward = availableInContract < roundReward
            ? availableInContract
            : roundReward;
        previousRoundIndex = currentRoundIndex;
        previousRoundTotalScores = 0;
        previousRoundRoundReward = currentRoundRoundReward;
        currentRoundIndex = currentRoundEndBlockNumber == 0
            ? 0
            : currentRoundIndex + 1;
        currentRoundEndBlockNumber = block.number + nextRoundLength;
        currentRoundRoundReward = nextAvailableRoundReward;
        emit RoundStart(currentRoundIndex);
    }

    function adminAdvanceRound() public onlyAdmin {
        advanceRound();
    }

    function setNextRoundLength(uint _nextRoundLength) public onlyAdmin {
        require(_nextRoundLength > 0, "Next round length must be positive");
        nextRoundLength = _nextRoundLength;
    }

    function setRoundReward(uint _roundReward) public onlyAdmin {
        roundReward = _roundReward;
    }

    function setMaxTransfersPerTx(uint _maxTransfersPerTx) public onlyAdmin {
        _setMaxTransfersPerTx(_maxTransfersPerTx);
    }

    function tick() public {
        if (block.number >= currentRoundEndBlockNumber) {
            advanceRound();
        }
        transferScheduled();
    }

    function addMeasurements(
        string calldata cid
    ) public virtual returns (uint) {
        uint measurementsRoundIndex = currentRoundIndex;
        emit MeasurementsAdded(cid, measurementsRoundIndex, msg.sender);
        tick();
        return measurementsRoundIndex;
    }
    
    function setScores(
        uint roundIndex,
        string calldata cid,
        uint totalScores
    ) public onlyEvaluator {
        require(
            previousRoundIndex != currentRoundIndex &&
                roundIndex == previousRoundIndex,
            "Can only score previous round"
        );
        require(
            totalScores <= MAX_SCORE,
            "Sum of scores including historic too big"
        );
        require(
            previousRoundTotalScores == 0,
            "Round already scored"
        );
        previousRoundTotalScores = totalScores;
        balanceHeld += (totalScores * previousRoundRoundReward) / MAX_SCORE;

        // `cid` points to a list of (address, score)
        emit Scores(roundIndex, cid);
    }

    function sendRewards(
      address payable[] calldata addresses,
      uint[] calldata amounts
    ) public onlyAdmin {
        require(
            addresses.length == amounts.length,
            "Addresses and amounts must be same length"
        );
        for (uint i = 0; i < addresses.length; i++) {
            scheduleTransfer(addresses[i], amounts[i]);
        }
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
