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
            1000
        );
        assertEq(impactEvaluator.currentRoundIndex(), 0);
        assertNotEq(impactEvaluator.getRound(0).end, 0);
        assertEq(impactEvaluator.getRound(0).end, block.number + 1000);
        vm.expectEmit(false, false, false, true);
        emit RoundStart(1);
        impactEvaluator.adminAdvanceRound();
        assertEq(impactEvaluator.currentRoundIndex(), 1);
    }

    function test_AddMeasurement() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(
            address(0x1),
            1000
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
            1000
        );
        vm.expectRevert("Not an evaluator");
        impactEvaluator.setScores(0, new address[](0), new uint[](0));
    }

    function test_SetScores() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(
            address(this),
            1000
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
        impactEvaluator.setScores(1, new address[](0), new uint[](0));
        vm.expectRevert("Addresses and scores length mismatch");
        impactEvaluator.setScores(0, new address[](1), new uint[](0));

        address[] memory addresses = new address[](1);
        addresses[0] = address(0x1);
        uint[] memory scores = new uint[](1);
        scores[0] = 1;
        impactEvaluator.setScores(0, addresses, scores);

        ImpactEvaluator.Round memory round = impactEvaluator.getRound(0);
        assertEq(round.participantAddresses.length, 1);
        assertEq(round.participantScores.length, 1);
        assertEq(round.participantAddresses[0], address(0x1));
        assertEq(round.participantScores[0], 1);
        assertEq(round.scoresSubmitted, true);

        vm.expectRevert("Scores already submitted");
        impactEvaluator.setScores(0, addresses, scores);
    }
}
