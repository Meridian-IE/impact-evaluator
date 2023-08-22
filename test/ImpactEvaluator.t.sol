// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "../src/ImpactEvaluator.sol";

contract ImpactEvaluatorTest is Test {
    event RoundStart(uint roundIndex);
    event MeasurementAdded(string cid, uint roundIndex);

    function test_AdvanceRound() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(
            address(this),
            1000,
            100
        );
        assertEq(impactEvaluator.currentRoundIndex(), 0);
        assertNotEq(impactEvaluator.getRound(0).start, 0);
        assertEq(impactEvaluator.getRound(0).start, block.number);
        vm.expectEmit(false, false, false, true);
        emit RoundStart(1);
        impactEvaluator.adminAdvanceRound();
        assertEq(impactEvaluator.currentRoundIndex(), 1);
    }

    function test_AdminSetRoundReward() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(
            address(this),
            1000,
            100
        );
        assertEq(impactEvaluator.roundReward(), 100);
        impactEvaluator.adminSetRoundReward(200);
        assertEq(impactEvaluator.roundReward(), 200);
    }

    function test_AdminSetRoundRewardNotAdmin() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(
            address(0x1),
            1000,
            100
        );
        vm.expectRevert("Not an admin");
        impactEvaluator.adminSetRoundReward(200);
    }

    function test_AddMeasurement() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(
            address(0x1),
            1000,
            100
        );
        assertEq(impactEvaluator.getRound(0).measurementCids.length, 0);
        vm.expectEmit(false, false, false, true);
        emit MeasurementAdded("cid", 0);
        impactEvaluator.addMeasurement("cid");
        assertEq(impactEvaluator.getRound(0).measurementCids.length, 1);
        assertEq(impactEvaluator.getRound(0).measurementCids[0], "cid");
    }

    function test_SetScoresNotEvaluator() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(
            address(0x1),
            1000,
            100
        );
        vm.expectRevert("Not an evaluator");
        impactEvaluator.setScores(
            0,
            new address payable[](0),
            new uint[](0),
            "no measurements"
        );
    }

    function test_SetScores() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(
            address(this),
            1000,
            100
        );
        impactEvaluator.adminAdvanceRound();
        impactEvaluator.grantRole(
            impactEvaluator.EVALUATE_ROLE(),
            address(this)
        );
        impactEvaluator.revokeRole(
            impactEvaluator.DEFAULT_ADMIN_ROLE(),
            address(this)
        );
        vm.expectRevert("Wrong round");
        impactEvaluator.setScores(
            1,
            new address payable[](0),
            new uint[](0),
            "no measurements"
        );
        vm.expectRevert("Addresses and scores length mismatch");
        impactEvaluator.setScores(
            0,
            new address payable[](1),
            new uint[](0),
            "one peer"
        );

        address payable[] memory addresses = new address payable[](1);
        addresses[0] = payable(vm.addr(1));
        uint[] memory scores = new uint[](1);
        scores[0] = 1000000;
        vm.deal(payable(address(impactEvaluator)), 100);
        impactEvaluator.setScores(0, addresses, scores, "1 task performed");
        assertEq(addresses[0].balance, 100);

        ImpactEvaluator.Round memory round = impactEvaluator.getRound(0);
        assertEq(round.participantAddresses.length, 1);
        assertEq(round.participantScores.length, 1);
        assertEq(round.participantAddresses[0], addresses[0]);
        assertEq(round.participantScores[0], scores[0]);
        assertEq(round.summaryText, "1 task performed");
        assertEq(round.scoresSubmitted, true);

        vm.expectRevert("Scores already submitted");
        impactEvaluator.setScores(0, addresses, scores, "1 task performed");
    }
}
