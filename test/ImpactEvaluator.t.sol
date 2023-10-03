// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "../src/ImpactEvaluator.sol";

contract ImpactEvaluatorTest is Test {
    event RoundStart(uint roundIndex);
    event MeasurementsAdded(string cid, uint roundIndex, address sender);
    event Transfer(address indexed to, uint256 amount);

    function test_AdvanceRound() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        assertEq(impactEvaluator.currentRoundIndex(), 0);
        assertEq(impactEvaluator.getRoundEnd(0), block.number + 10);
        vm.expectEmit(false, false, false, true);
        emit RoundStart(1);
        impactEvaluator.adminAdvanceRound();
        assertEq(impactEvaluator.currentRoundIndex(), 1);
    }

    function test_AdvanceRoundCleanup() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        impactEvaluator.setMaxStoredRounds(1);
        impactEvaluator.adminAdvanceRound();
        assertEq(impactEvaluator.currentRoundIndex(), 1);
        assertEq(impactEvaluator.getRoundExists(0), false);
        assertEq(impactEvaluator.getRoundExists(1), true);
        impactEvaluator.setMaxStoredRounds(1000);
        impactEvaluator.adminAdvanceRound();
        assertEq(impactEvaluator.getRoundExists(0), false);
        assertEq(impactEvaluator.getRoundExists(1), true);
        assertEq(impactEvaluator.getRoundExists(2), true);
        impactEvaluator.setMaxStoredRounds(1);
        assertEq(impactEvaluator.getRoundExists(0), false);
        assertEq(impactEvaluator.getRoundExists(1), false);
        assertEq(impactEvaluator.getRoundExists(2), true);
    }

    function test_SetNextRoundLength() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        assertEq(impactEvaluator.getRoundEnd(0), block.number + 10);
        impactEvaluator.setNextRoundLength(20);
        assertEq(impactEvaluator.getRoundEnd(0), block.number + 10);
        impactEvaluator.adminAdvanceRound();
        assertEq(impactEvaluator.getRoundEnd(1), block.number + 20);
    }

    function test_setRoundReward() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        assertEq(impactEvaluator.roundReward(), 100 ether);
        impactEvaluator.setRoundReward(200 ether);
        assertEq(impactEvaluator.roundReward(), 200 ether);
    }

    function test_setRoundRewardNotAdmin() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(0x1));
        vm.expectRevert("Not an admin");
        impactEvaluator.setRoundReward(200 ether);
    }

    function test_AddMeasurements() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(0x1));
        assertEq(impactEvaluator.getRoundMeasurementsCids(0).length, 0);
        vm.expectEmit(false, false, false, true);
        emit MeasurementsAdded("cid", 0, address(this));
        uint roundIndex = impactEvaluator.addMeasurements("cid");
        assertEq(roundIndex, 0);
        assertEq(impactEvaluator.getRoundMeasurementsCids(0).length, 1);
        assertEq(impactEvaluator.getRoundMeasurementsCids(0)[0], "cid");
    }

    function test_SetScoresNotEvaluator() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(0x1));
        vm.expectRevert("Not an evaluator");
        impactEvaluator.setScores(
            0,
            new address payable[](0),
            new uint64[](0),
            "no measurements"
        );
    }

    function test_SetScores() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        impactEvaluator.adminAdvanceRound();
        impactEvaluator.revokeRole(
            impactEvaluator.DEFAULT_ADMIN_ROLE(),
            address(this)
        );
        vm.expectRevert("Addresses and scores length mismatch");
        impactEvaluator.setScores(
            0,
            new address payable[](1),
            new uint64[](0),
            "one peer"
        );

        address payable[] memory addresses = new address payable[](1);
        addresses[0] = payable(vm.addr(1));
        uint64[] memory scores = new uint64[](1);
        scores[0] = 1e15;
        vm.deal(payable(address(impactEvaluator)), 100 ether);
        vm.expectEmit(false, false, false, true);
        emit Transfer(addresses[0], 100 ether);
        impactEvaluator.setScores(0, addresses, scores, "1 task performed");
        assertEq(addresses[0].balance, 100 ether, "correct balance");

        assertEq(impactEvaluator.getParticipantScore(0, addresses[0]), scores[0]);
        assertEq(impactEvaluator.getRoundSummaryText(0), "1 task performed");
        assertEq(impactEvaluator.getRoundScoresSubmitted(0), true);

        vm.expectRevert("Scores already submitted");
        impactEvaluator.setScores(0, addresses, scores, "1 task performed");
    }

    function test_SetScoresMultipleParticipants() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        impactEvaluator.adminAdvanceRound();

        address payable[] memory addresses = new address payable[](3);
        addresses[0] = payable(vm.addr(1));
        addresses[1] = payable(vm.addr(2));
        addresses[2] = payable(vm.addr(3));

        uint64[] memory scores = new uint64[](3);
        scores[0] = 50e13;
        scores[1] = 25e13;
        scores[2] = 25e13;
        vm.deal(payable(address(impactEvaluator)), 100 ether);
        impactEvaluator.setScores(0, addresses, scores, "some task performed");
        assertEq(addresses[0].balance, 50 ether, "addresses[0] balance");
        assertEq(addresses[1].balance, 25 ether);
        assertEq(addresses[2].balance, 25 ether);
    }

    function test_SetScoresFractions() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        impactEvaluator.adminAdvanceRound();

        address payable[] memory addresses = new address payable[](2);
        addresses[0] = payable(vm.addr(1));
        addresses[1] = payable(vm.addr(2));
        uint64[] memory scores = new uint64[](2);
        scores[0] = 1e15 - 1;
        scores[1] = 1;
        vm.deal(payable(address(impactEvaluator)), 100 ether);
        vm.expectEmit(false, false, false, true);
        emit Transfer(addresses[0], 100 ether - 1e5);
        vm.expectEmit(false, false, false, true);
        emit Transfer(addresses[0], 1e5);
        impactEvaluator.setScores(0, addresses, scores, "2 tasks performed");
        assertEq(addresses[0].balance, 100 ether - 1e5, "addresses[0] balance");
        assertEq(addresses[1].balance, 1e5, "addresses[1] balance");
    }

    function test_SetScoresEmptyRound() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        impactEvaluator.adminAdvanceRound();

        address payable[] memory addresses = new address payable[](0);
        uint64[] memory scores = new uint64[](0);
        vm.deal(payable(address(impactEvaluator)), 100 ether);
        impactEvaluator.setScores(0, addresses, scores, "0 tasks performed");
    }

    function test_CurrentRoundMeasurementCount() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        assertEq(impactEvaluator.currentRoundMeasurementCount(), 0);
        impactEvaluator.addMeasurements("cid");
        assertEq(impactEvaluator.currentRoundMeasurementCount(), 1);
        impactEvaluator.adminAdvanceRound();
        assertEq(impactEvaluator.currentRoundMeasurementCount(), 0);
    }

    function testSetMaxStoredRounds() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        assertEq(impactEvaluator.maxStoredRounds(), 1000);
        impactEvaluator.setMaxStoredRounds(500);
        assertEq(impactEvaluator.maxStoredRounds(), 500);
        impactEvaluator.setMaxStoredRounds(2000);
        assertEq(impactEvaluator.maxStoredRounds(), 2000);
    }
}
