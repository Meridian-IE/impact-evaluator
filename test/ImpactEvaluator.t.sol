// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "../src/ImpactEvaluator.sol";

contract ImpactEvaluatorTest is Test {
    event RoundStart(uint roundIndex);
    event MeasurementAdded(string cid, string provider);

    function test_AdvanceRound() public {
        address[] memory evaluators = new address[](1);
        evaluators[0] = address(0x1);
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(
            address(this),
            evaluators
        );
        assertEq(impactEvaluator.currentRoundIndex(), 0);
        vm.expectEmit(false, false, false, true);
        emit RoundStart(1);
        impactEvaluator.adminAdvanceRound();
        assertEq(impactEvaluator.currentRoundIndex(), 1);
    }

    function test_AddMeasurement() public {
        address[] memory evaluators = new address[](1);
        evaluators[0] = address(0x1);
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(
            address(0x1),
            evaluators
        );
        assertEq(impactEvaluator.getRound(0).measurementCids.length, 0);
        assertEq(impactEvaluator.getRound(0).measurementProviders.length, 0);
        vm.expectEmit(false, false, false, true);
        emit MeasurementAdded("cid", "provider");
        impactEvaluator.addMeasurement("cid", "provider");
        assertEq(impactEvaluator.getRound(0).measurementCids.length, 1);
        assertEq(impactEvaluator.getRound(0).measurementProviders.length, 1);
        assertEq(impactEvaluator.getRound(0).measurementCids[0], "cid");
        assertEq(
            impactEvaluator.getRound(0).measurementProviders[0],
            "provider"
        );
    }
}
