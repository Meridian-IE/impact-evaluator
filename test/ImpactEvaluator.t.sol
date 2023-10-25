// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "../src/ImpactEvaluator.sol";

contract ImpactEvaluatorTest is Test {
    event RoundStart(uint roundIndex);
    event MeasurementsAdded(
        string cid,
        uint roundIndex,
        address indexed sender
    );
    event Transfer(address indexed to, uint256 amount);

    function test_AdvanceRound() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        assertEq(impactEvaluator.currentRoundIndex(), 0);
        (, uint end, ) = impactEvaluator.openRounds(0);
        assertEq(end, block.number + 10);
        vm.expectEmit(false, false, false, true);
        emit RoundStart(1);
        impactEvaluator.adminAdvanceRound();
        assertEq(impactEvaluator.currentRoundIndex(), 1);
    }

    function test_SetNextRoundLength() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        (, uint end,) = impactEvaluator.openRounds(0);
        assertEq(end, block.number + 10);
        impactEvaluator.setNextRoundLength(20);
        (, end, ) = impactEvaluator.openRounds(0);
        assertEq(end, block.number + 10);
        impactEvaluator.adminAdvanceRound();
        (, end, ) = impactEvaluator.openRounds(1);
        assertEq(end, block.number + 20);
        vm.expectRevert("Next round length must be positive");
        impactEvaluator.setNextRoundLength(0);
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
        vm.expectEmit(false, false, false, true);
        emit MeasurementsAdded("cid", 0, address(this));
        uint roundIndex = impactEvaluator.addMeasurements("cid");
        assertEq(roundIndex, 0);
    }

    function test_SetScoresNotEvaluator() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(0x1));
        vm.expectRevert("Not an evaluator");
        impactEvaluator.setScores(0, new address payable[](0), new uint64[](0));
    }

    function test_SetScores() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        impactEvaluator.adminAdvanceRound();
        impactEvaluator.revokeRole(
            impactEvaluator.DEFAULT_ADMIN_ROLE(),
            address(this)
        );
        vm.expectRevert("Addresses and scores length mismatch");
        impactEvaluator.setScores(0, new address payable[](1), new uint64[](0));

        address payable[] memory addresses = new address payable[](1);
        addresses[0] = payable(vm.addr(1));
        uint64[] memory scores = new uint64[](1);
        scores[0] = impactEvaluator.MAX_SCORE();
        vm.deal(payable(address(impactEvaluator)), 100 ether);
        vm.expectEmit(false, false, false, true);
        emit Transfer(addresses[0], 100 ether);
        impactEvaluator.setScores(0, addresses, scores);
        assertEq(addresses[0].balance, 100 ether, "correct balance");

        vm.expectRevert("Open round does not exist");
        impactEvaluator.setScores(0, addresses, scores);
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
        impactEvaluator.setScores(0, addresses, scores);
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
        scores[0] = impactEvaluator.MAX_SCORE() - 1;
        scores[1] = 1;
        vm.deal(payable(address(impactEvaluator)), 100 ether);
        vm.expectEmit(false, false, false, true);
        emit Transfer(addresses[0], 100 ether - 1e5);
        vm.expectEmit(false, false, false, true);
        emit Transfer(addresses[0], 1e5);
        impactEvaluator.setScores(0, addresses, scores);
        assertEq(addresses[0].balance, 100 ether - 1e5, "addresses[0] balance");
        assertEq(addresses[1].balance, 1e5, "addresses[1] balance");
    }

    function test_SetScoresEmptyRound() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        impactEvaluator.adminAdvanceRound();

        address payable[] memory addresses = new address payable[](0);
        uint64[] memory scores = new uint64[](0);
        vm.deal(payable(address(impactEvaluator)), 100 ether);
        impactEvaluator.setScores(0, addresses, scores);
    }

    function test_SetScoresMultipleCalls() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        impactEvaluator.adminAdvanceRound();
        vm.deal(payable(address(impactEvaluator)), 100 ether);
        uint64 iterations = 10;
        for (uint i = 0; i < iterations; i++) {
            address payable[] memory addresses = new address payable[](1);
            addresses[0] = payable(vm.addr(i + 1));
            uint64[] memory scores = new uint64[](1);
            scores[0] = impactEvaluator.MAX_SCORE() / iterations;
            impactEvaluator.setScores(0, addresses, scores);
        }
        for (uint i = 0; i < iterations; i++) {
            assertEq(
                vm.addr(i + 1).balance,
                100 ether / iterations,
                "right balance"
            );
        }
    }

    function test_SetScoresTooBig() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        impactEvaluator.adminAdvanceRound();

        address payable[] memory addresses = new address payable[](1);
        addresses[0] = payable(vm.addr(1));
        uint64[] memory scores = new uint64[](1);
        scores[0] = impactEvaluator.MAX_SCORE() + 1;
        vm.deal(payable(address(impactEvaluator)), 100 ether);
        vm.expectRevert("Sum of scores too big");
        impactEvaluator.setScores(0, addresses, scores);
    }

    function test_SetScoresTooBigHistoric() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        impactEvaluator.adminAdvanceRound();
        vm.deal(payable(address(impactEvaluator)), 100 ether);

        address payable[] memory addresses = new address payable[](1);
        addresses[0] = payable(vm.addr(1));
        uint64[] memory scores = new uint64[](1);
        scores[0] = impactEvaluator.MAX_SCORE() - 1;
        impactEvaluator.setScores(0, addresses, scores);
    
        scores[0] = 2;
        vm.expectRevert("Sum of scores including historic too big");
        impactEvaluator.setScores(0, addresses, scores);
    }

    function test_SetScoresOverflow() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        impactEvaluator.adminAdvanceRound();
        vm.deal(payable(address(impactEvaluator)), 100 ether);

        address payable[] memory addresses = new address payable[](2);
        uint64[] memory scores = new uint64[](2);
        addresses[0] = payable(vm.addr(1));
        addresses[1] = payable(vm.addr(1));
        scores[0] = 2**64 - 1;
        scores[1] = 1;
        vm.expectRevert();
        impactEvaluator.setScores(0, addresses, scores);
    }

    function test_SetScoresUnfinishedRound() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        address payable[] memory addresses = new address payable[](0);
        uint64[] memory scores = new uint64[](0);
        vm.expectRevert("Round not finished");
        impactEvaluator.setScores(0, addresses, scores);
    }

    function test_SetScoresBurner() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        impactEvaluator.adminAdvanceRound();
        vm.deal(payable(address(impactEvaluator)), 100 ether);

        address payable[] memory addresses = new address payable[](1);
        uint64[] memory scores = new uint64[](1);
        addresses[0] = payable(0x000000000000000000000000000000000000dEaD);
        scores[0] = impactEvaluator.MAX_SCORE();
        impactEvaluator.setScores(0, addresses, scores);
        assertEq(address(impactEvaluator).balance, 100 ether, "correct balance");
        assertEq(addresses[0].balance, 0, "correct balance");
    }

    function test_AdminDeleteOpenRound() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(vm.addr(1)));
        vm.expectRevert("Not an admin");
        impactEvaluator.adminDeleteOpenRound(0);

        impactEvaluator = new ImpactEvaluator(address(this));
        vm.expectRevert("Round not finished");
        impactEvaluator.adminDeleteOpenRound(0);

        impactEvaluator.adminAdvanceRound();
        impactEvaluator.adminDeleteOpenRound(0);
    }
}
