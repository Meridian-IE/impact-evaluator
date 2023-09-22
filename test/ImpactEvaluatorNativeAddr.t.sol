// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "../src/ImpactEvaluatorNativeAddr.sol";
import "../lib/filecoin-solidity/contracts/v0.8/utils/FilAddresses.sol";

contract ImpactEvaluatorNativeAddrTest is Test {
    event RoundStart(uint roundIndex);
    event MeasurementsAdded(string cid, uint roundIndex, address sender);
    event Transfer(CommonTypes.FilAddress indexed to, uint256 amount);

    function test_AdvanceRound() public {
        ImpactEvaluatorNativeAddr impactEvaluator = new ImpactEvaluatorNativeAddr(address(this));
        assertEq(impactEvaluator.currentRoundIndex(), 0);
        assertEq(impactEvaluator.getRoundEnd(0), block.number + 10);
        vm.expectEmit(false, false, false, true);
        emit RoundStart(1);
        impactEvaluator.adminAdvanceRound();
        assertEq(impactEvaluator.currentRoundIndex(), 1);
    }

    function test_AdvanceRoundCleanup() public {
        ImpactEvaluatorNativeAddr impactEvaluator = new ImpactEvaluatorNativeAddr(address(this));
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
        ImpactEvaluatorNativeAddr impactEvaluator = new ImpactEvaluatorNativeAddr(address(this));
        assertEq(impactEvaluator.getRoundEnd(0), block.number + 10);
        impactEvaluator.setNextRoundLength(20);
        assertEq(impactEvaluator.getRoundEnd(0), block.number + 10);
        impactEvaluator.adminAdvanceRound();
        assertEq(impactEvaluator.getRoundEnd(1), block.number + 20);
    }

    function test_setRoundReward() public {
        ImpactEvaluatorNativeAddr impactEvaluator = new ImpactEvaluatorNativeAddr(address(this));
        assertEq(impactEvaluator.roundReward(), 100);
        impactEvaluator.setRoundReward(200);
        assertEq(impactEvaluator.roundReward(), 200);
    }

    function test_setRoundRewardNotAdmin() public {
        ImpactEvaluatorNativeAddr impactEvaluator = new ImpactEvaluatorNativeAddr(address(0x1));
        vm.expectRevert("Not an admin");
        impactEvaluator.setRoundReward(200);
    }

    function test_AddMeasurements() public {
        ImpactEvaluatorNativeAddr impactEvaluator = new ImpactEvaluatorNativeAddr(address(0x1));
        assertEq(impactEvaluator.getRoundMeasurementsCids(0).length, 0);
        vm.expectEmit(false, false, false, true);
        emit MeasurementsAdded("cid", 0, address(this));
        uint roundIndex = impactEvaluator.addMeasurements("cid");
        assertEq(roundIndex, 0);
        assertEq(impactEvaluator.getRoundMeasurementsCids(0).length, 1);
        assertEq(impactEvaluator.getRoundMeasurementsCids(0)[0], "cid");
    }

    function test_SetScoresNotEvaluator() public {
        ImpactEvaluatorNativeAddr impactEvaluator = new ImpactEvaluatorNativeAddr(address(0x1));
        vm.expectRevert("Not an evaluator");
        impactEvaluator.setScores(
            0,
            new CommonTypes.FilAddress[](0),
            new uint64[](0),
            "no measurements"
        );
    }

    function test_SetScores() public {
        ImpactEvaluatorNativeAddr impactEvaluator = new ImpactEvaluatorNativeAddr(address(this));
        impactEvaluator.adminAdvanceRound();
        impactEvaluator.revokeRole(
            impactEvaluator.DEFAULT_ADMIN_ROLE(),
            address(this)
        );
        vm.expectRevert("Addresses and scores length mismatch");
        impactEvaluator.setScores(
            0,
            new CommonTypes.FilAddress[](1),
            new uint64[](0),
            "one peer"
        );

        CommonTypes.FilAddress[] memory addresses = new CommonTypes.FilAddress[](1);
        addresses[0] = FilAddresses.fromEthAddress(vm.addr(1));
        uint64[] memory scores = new uint64[](1);
        scores[0] = 1000000000000000;
        vm.deal(payable(address(impactEvaluator)), 100);
        // vm.expectEmit(false, false, false, true);
        // emit Transfer(addresses[0], 100);
        impactEvaluator.setScores(0, addresses, scores, "1 task performed");
        // TODO: assertEq(addresses[0].balance, 100);

        assertEq(impactEvaluator.getScores(0)[0], scores[0]);
        assertEq(impactEvaluator.getRoundSummaryText(0), "1 task performed");
        assertEq(impactEvaluator.getRoundScoresSubmitted(0), true);

        vm.expectRevert("Scores already submitted");
        impactEvaluator.setScores(0, addresses, scores, "1 task performed");
    }

    function test_SetScoresEmptyRound() public {
        ImpactEvaluatorNativeAddr impactEvaluator = new ImpactEvaluatorNativeAddr(address(this));
        impactEvaluator.adminAdvanceRound();
        impactEvaluator.revokeRole(
            impactEvaluator.DEFAULT_ADMIN_ROLE(),
            address(this)
        );

        CommonTypes.FilAddress[] memory addresses = new CommonTypes.FilAddress[](0);
        uint64[] memory scores = new uint64[](0);
        vm.deal(payable(address(impactEvaluator)), 100);
        impactEvaluator.setScores(0, addresses, scores, "0 tasks performed");
    }

    function test_CurrentRoundMeasurementCount() public {
        ImpactEvaluatorNativeAddr impactEvaluator = new ImpactEvaluatorNativeAddr(address(this));
        assertEq(impactEvaluator.currentRoundMeasurementCount(), 0);
        impactEvaluator.addMeasurements("cid");
        assertEq(impactEvaluator.currentRoundMeasurementCount(), 1);
        impactEvaluator.adminAdvanceRound();
        assertEq(impactEvaluator.currentRoundMeasurementCount(), 0);
    }
}
