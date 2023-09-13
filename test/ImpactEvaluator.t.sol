// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "../src/ImpactEvaluator.sol";

contract ImpactEvaluatorTest is Test {
    event RoundStart(uint roundIndex);
    event MeasurementsAdded(string cid, uint roundIndex);
    event Transfer(address indexed to, uint256 amount);

    function test_AdvanceRound() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        assertEq(impactEvaluator.currentRoundIndex(), 0);
        assertEq(impactEvaluator.getRound(0).end, block.number + 10);
        vm.expectEmit(false, false, false, true);
        emit RoundStart(1);
        impactEvaluator.adminAdvanceRound();
        assertEq(impactEvaluator.currentRoundIndex(), 1);
    }

    function test_SetNextRoundLength() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        assertEq(impactEvaluator.getRound(0).end, block.number + 10);
        impactEvaluator.setNextRoundLength(20);
        assertEq(impactEvaluator.getRound(0).end, block.number + 10);
        impactEvaluator.adminAdvanceRound();
        assertEq(impactEvaluator.getRound(1).end, block.number + 20);
    }

    function test_setRoundReward() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        assertEq(impactEvaluator.roundReward(), 100);
        impactEvaluator.setRoundReward(200);
        assertEq(impactEvaluator.roundReward(), 200);
    }

    function test_setRoundRewardNotAdmin() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(0x1));
        vm.expectRevert("Not an admin");
        impactEvaluator.setRoundReward(200);
    }

    function test_AddMeasurements() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(0x1));
        assertEq(impactEvaluator.getRound(0).measurementsCids.length, 0);
        vm.expectEmit(false, false, false, true);
        emit MeasurementsAdded("cid", 0);
        uint roundIndex = impactEvaluator.addMeasurements("cid");
        assertEq(roundIndex, 0);
        assertEq(impactEvaluator.getRound(0).measurementsCids.length, 1);
        assertEq(impactEvaluator.getRound(0).measurementsCids[0], "cid");
    }

    function test_SetScoresNotEvaluator() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(0x1));
        vm.expectRevert("Not an evaluator");
        impactEvaluator.setScores(
            0,
            new address payable[](0),
            new uint[](0),
            "no measurements"
        );
    }

    function test_SetScores() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
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
        scores[0] = 1000000000000000;
        vm.deal(payable(address(impactEvaluator)), 100);
        vm.expectEmit(false, false, false, true);
        emit Transfer(addresses[0], 100);
        impactEvaluator.setScores(0, addresses, scores, "1 task performed");
        assertEq(addresses[0].balance, 100);

        ImpactEvaluator.Round memory round = impactEvaluator.getRound(0);
        assertEq(round.participents.length, 1);
        assertEq(round.scores.length, 1);
        assertEq(round.participents[0], addresses[0]);
        assertEq(round.scores[0], scores[0]);
        assertEq(round.summaryText, "1 task performed");
        assertEq(round.scoresSubmitted, true);

        vm.expectRevert("Scores already submitted");
        impactEvaluator.setScores(0, addresses, scores, "1 task performed");
    }

    function test_SetScoresEmptyRound() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        impactEvaluator.adminAdvanceRound();
        impactEvaluator.grantRole(
            impactEvaluator.EVALUATE_ROLE(),
            address(this)
        );
        impactEvaluator.revokeRole(
            impactEvaluator.DEFAULT_ADMIN_ROLE(),
            address(this)
        );

        address payable[] memory addresses = new address payable[](0);
        uint[] memory scores = new uint[](0);
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
}
