// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "../src/ImpactEvaluator.sol";
import "../lib/openzeppelin-contracts/contracts/utils/Strings.sol";

contract ImpactEvaluatorTest is Test {
    event RoundStart(uint roundIndex);
    event MeasurementsAdded(
        string cid,
        uint indexed roundIndex,
        address indexed sender
    );

    function test_AdvanceRound() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        assertEq(impactEvaluator.currentRoundIndex(), 0);
        assertEq(
            impactEvaluator.currentRoundEndBlockNumber(),
            block.number + 10
        );
        vm.expectEmit(false, false, false, true);
        emit RoundStart(1);
        impactEvaluator.adminAdvanceRound();
        assertEq(impactEvaluator.currentRoundIndex(), 1);
    }

    function test_SetNextRoundLength() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        assertEq(
            impactEvaluator.currentRoundEndBlockNumber(),
            block.number + 10
        );
        impactEvaluator.setNextRoundLength(20);
        assertEq(
            impactEvaluator.currentRoundEndBlockNumber(),
            block.number + 10
        );
        impactEvaluator.adminAdvanceRound();
        assertEq(
            impactEvaluator.currentRoundEndBlockNumber(),
            block.number + 20
        );
        vm.expectRevert("Next round length must be positive");
        impactEvaluator.setNextRoundLength(0);
    }

    function test_SetRoundReward() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        assertEq(impactEvaluator.roundReward(), 100 ether);
        impactEvaluator.setRoundReward(200 ether);
        assertEq(impactEvaluator.roundReward(), 200 ether);
    }

    function test_SetRoundRewardNotAdmin() public {
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
        impactEvaluator.setScores(0, new address payable[](0), new uint[](0));
    }

    function test_SetScores() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        vm.deal(payable(address(impactEvaluator)), 100 ether);
        impactEvaluator.adminAdvanceRound();
        impactEvaluator.adminAdvanceRound();
        impactEvaluator.revokeRole(
            impactEvaluator.DEFAULT_ADMIN_ROLE(),
            address(this)
        );
        vm.expectRevert("Addresses and scores length mismatch");
        impactEvaluator.setScores(1, new address payable[](1), new uint[](0));

        address payable[] memory addresses = new address payable[](1);
        addresses[0] = payable(vm.addr(1));
        uint[] memory scores = new uint[](1);
        scores[0] = impactEvaluator.MAX_SCORE();
        impactEvaluator.setScores(1, addresses, scores);
        assertEq(
            impactEvaluator.rewardsScheduledFor(addresses[0]),
            100 ether,
            "correct balance"
        );

        vm.expectRevert("Can only score previous round");
        impactEvaluator.setScores(0, addresses, scores);
    }

    function test_SetScoresMultipleParticipants() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        vm.deal(payable(address(impactEvaluator)), 100 ether);
        impactEvaluator.adminAdvanceRound();
        impactEvaluator.adminAdvanceRound();

        address payable[] memory addresses = new address payable[](3);
        addresses[0] = payable(vm.addr(1));
        addresses[1] = payable(vm.addr(2));
        addresses[2] = payable(vm.addr(3));

        uint[] memory scores = new uint[](3);
        scores[0] = 50e13;
        scores[1] = 25e13;
        scores[2] = 25e13;
        impactEvaluator.setScores(1, addresses, scores);
        assertEq(
            impactEvaluator.rewardsScheduledFor(addresses[0]),
            50 ether,
            "addresses[0] balance"
        );
        assertEq(impactEvaluator.rewardsScheduledFor(addresses[1]), 25 ether);
        assertEq(impactEvaluator.rewardsScheduledFor(addresses[2]), 25 ether);
    }

    function test_SetScoresFractions() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        vm.deal(payable(address(impactEvaluator)), 100 ether);
        impactEvaluator.adminAdvanceRound();
        impactEvaluator.adminAdvanceRound();

        address payable[] memory addresses = new address payable[](2);
        addresses[0] = payable(vm.addr(1));
        addresses[1] = payable(vm.addr(2));
        uint[] memory scores = new uint[](2);
        scores[0] = impactEvaluator.MAX_SCORE() - 1;
        scores[1] = 1;
        impactEvaluator.setScores(1, addresses, scores);
        assertEq(
            impactEvaluator.rewardsScheduledFor(addresses[0]),
            100 ether - 1e5,
            "addresses[0] balance"
        );
        assertEq(
            impactEvaluator.rewardsScheduledFor(addresses[1]),
            1e5,
            "addresses[1] balance"
        );
    }

    function test_SetScoresEmptyRound() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        impactEvaluator.adminAdvanceRound();

        address payable[] memory addresses = new address payable[](0);
        uint[] memory scores = new uint[](0);
        impactEvaluator.setScores(0, addresses, scores);
    }

    function test_SetScoresMultipleCalls() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        vm.deal(payable(address(impactEvaluator)), 100 ether);
        impactEvaluator.adminAdvanceRound();
        impactEvaluator.adminAdvanceRound();
        uint iterations = 10;
        for (uint i = 0; i < iterations; i++) {
            address payable[] memory addresses = new address payable[](1);
            addresses[0] = payable(vm.addr(i + 1));
            uint[] memory scores = new uint[](1);
            scores[0] = impactEvaluator.MAX_SCORE() / iterations;
            impactEvaluator.setScores(1, addresses, scores);
        }
        for (uint i = 0; i < iterations; i++) {
            assertEq(
                impactEvaluator.rewardsScheduledFor(vm.addr(i + 1)),
                100 ether / iterations,
                string.concat("address[", Strings.toString(i), "] balance")
            );
        }
    }

    function test_SetScoresTooBig() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        impactEvaluator.adminAdvanceRound();

        address payable[] memory addresses = new address payable[](1);
        addresses[0] = payable(vm.addr(1));
        uint[] memory scores = new uint[](1);
        scores[0] = impactEvaluator.MAX_SCORE() + 1;
        vm.expectRevert("Sum of scores including historic too big");
        impactEvaluator.setScores(0, addresses, scores);
    }

    function test_SetScoresTooBigHistoric() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        impactEvaluator.adminAdvanceRound();

        address payable[] memory addresses = new address payable[](1);
        addresses[0] = payable(vm.addr(1));
        uint[] memory scores = new uint[](1);
        scores[0] = impactEvaluator.MAX_SCORE() - 1;
        impactEvaluator.setScores(0, addresses, scores);

        scores[0] = 2;
        vm.expectRevert("Sum of scores including historic too big");
        impactEvaluator.setScores(0, addresses, scores);
    }

    function test_SetScoresOverflow() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        impactEvaluator.adminAdvanceRound();

        address payable[] memory addresses = new address payable[](2);
        uint[] memory scores = new uint[](2);
        addresses[0] = payable(vm.addr(1));
        addresses[1] = payable(vm.addr(1));
        scores[0] = 2 ** 64 - 1;
        scores[1] = 1;
        vm.expectRevert();
        impactEvaluator.setScores(0, addresses, scores);
    }

    function test_SetScoresUnfinishedRound() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        address payable[] memory addresses = new address payable[](0);
        uint[] memory scores = new uint[](0);
        vm.expectRevert("Can only score previous round");
        impactEvaluator.setScores(0, addresses, scores);
    }

    function test_AdvanceRoundCleanUp() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));

        impactEvaluator.adminAdvanceRound();

        address payable[] memory addresses = new address payable[](1);
        uint[] memory scores = new uint[](1);
        addresses[0] = payable(vm.addr(1));
        scores[0] = impactEvaluator.MAX_SCORE();
        impactEvaluator.setScores(0, addresses, scores);

        impactEvaluator.addMeasurements("cid");
    }

    function test_rewardsScheduledFor() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        assertEq(
            impactEvaluator.rewardsScheduledFor(address(this)),
            0,
            "initial balance"
        );
        vm.deal(payable(address(impactEvaluator)), 100 ether);

        impactEvaluator.adminAdvanceRound();
        impactEvaluator.adminAdvanceRound();

        address payable[] memory addresses = new address payable[](1);
        addresses[0] = payable(address(this));
        uint[] memory scores = new uint[](1);
        scores[0] = impactEvaluator.MAX_SCORE();
        impactEvaluator.setScores(1, addresses, scores);

        assertEq(
            impactEvaluator.rewardsScheduledFor(address(this)),
            100 ether,
            "final balance"
        );
    }

    function test_Reward() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        vm.deal(payable(address(impactEvaluator)), 150 ether);
        impactEvaluator.adminAdvanceRound();
        impactEvaluator.adminAdvanceRound();

        address payable[] memory addresses = new address payable[](1);
        addresses[0] = payable(vm.addr(1));
        uint[] memory scores = new uint[](1);
        scores[0] = impactEvaluator.MAX_SCORE();

        impactEvaluator.setScores(1, addresses, scores);
        assertEq(
            impactEvaluator.rewardsScheduledFor(addresses[0]),
            100 ether,
            "full reward"
        );

        impactEvaluator.adminAdvanceRound();
        impactEvaluator.setScores(2, addresses, scores);
        assertEq(
            impactEvaluator.rewardsScheduledFor(addresses[0]),
            150 ether,
            "remaining reward"
        );

        impactEvaluator.adminAdvanceRound();
        impactEvaluator.setScores(3, addresses, scores);
        assertEq(
            impactEvaluator.rewardsScheduledFor(addresses[0]),
            150 ether,
            "no extra reward"
        );
    }

    function test_RewardBurner() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        // round 0: 0 ether available
        vm.deal(payable(address(impactEvaluator)), 100 ether);
        impactEvaluator.adminAdvanceRound();
        // round 1: 100 ether available
        impactEvaluator.adminAdvanceRound();
        // round 2: 0 ether available

        address payable[] memory addresses = new address payable[](1);
        addresses[0] = payable(0x000000000000000000000000000000000000dEaD);
        uint[] memory scores = new uint[](1);
        scores[0] = impactEvaluator.MAX_SCORE();

        impactEvaluator.setScores(1, addresses, scores);

        impactEvaluator.adminAdvanceRound();
        // round 3: 100 ether available
        impactEvaluator.adminAdvanceRound();
        // round 4: 0 ether available

        addresses[0] = payable(address(this));
        impactEvaluator.setScores(3, addresses, scores);
        assertEq(
            impactEvaluator.rewardsScheduledFor(address(this)),
            100 ether,
            "burner reward was added back to the pool"
        );
    }

    function test_ReleaseReward() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        vm.deal(payable(address(impactEvaluator)), 100 ether);

        impactEvaluator.adminAdvanceRound();
        impactEvaluator.adminAdvanceRound();

        impactEvaluator.releaseRewards();
        assertEq(vm.addr(1).balance, 0);

        address payable[] memory addresses = new address payable[](1);
        addresses[0] = payable(vm.addr(1));
        uint[] memory scores = new uint[](1);
        scores[0] = impactEvaluator.MAX_SCORE();
        impactEvaluator.setScores(1, addresses, scores);
        assertEq(vm.addr(1).balance, 0);
        impactEvaluator.releaseRewards();
        assertEq(vm.addr(1).balance, 100 ether);

        impactEvaluator.revokeRole(
            impactEvaluator.DEFAULT_ADMIN_ROLE(),
            address(this)
        );
        vm.expectRevert("Not an admin");
        impactEvaluator.releaseRewards();
    }

    function test_TransferScheduled() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        vm.deal(payable(address(impactEvaluator)), 100 ether);
        impactEvaluator.setMaxTransfersPerTx(5);

        impactEvaluator.adminAdvanceRound();
        impactEvaluator.adminAdvanceRound();

        uint participants = 10;
        address payable[] memory addresses = new address payable[](
            participants
        );
        uint[] memory scores = new uint[](participants);
        for (uint i = 0; i < participants; i++) {
            addresses[i] = payable(vm.addr(i + 1));
            scores[i] = impactEvaluator.MAX_SCORE() / participants;
        }
        impactEvaluator.setScores(1, addresses, scores);
        assertEq(impactEvaluator.participantCountReadyForTransfer(), 10);
        assertEq(impactEvaluator.participantCountScheduledForTransfer(), 0);
        for (uint i = 0; i < participants; i++) {
            assertEq(impactEvaluator.readyForTransfer(i), vm.addr(i + 1));
            vm.expectRevert();
            impactEvaluator.scheduledForTransfer(i);
        }

        impactEvaluator.releaseRewards();
        vm.expectRevert("Scheduled transfers still pending");
        impactEvaluator.releaseRewards();

        assertEq(impactEvaluator.participantCountReadyForTransfer(), 0);
        for (uint i = 0; i < 10; i++) {
            vm.expectRevert();
            impactEvaluator.readyForTransfer(i);
        }
        assertEq(impactEvaluator.participantCountScheduledForTransfer(), 5);
        for (uint i = 0; i < 5; i++) {
            assertEq(impactEvaluator.scheduledForTransfer(i), vm.addr(i + 1));
        }
        vm.expectRevert();
        impactEvaluator.scheduledForTransfer(5);
        for (uint i = 0; i < 5; i++) {
            assertEq(
                vm.addr(participants - i).balance,
                10 ether,
                string.concat(
                    "paid participant ",
                    Strings.toString(participants - i - 1),
                    " in first tick"
                )
            );
        }
        assertEq(vm.addr(5).balance, 0, "didn't pay more than 5 participants");

        impactEvaluator.tick();
        assertEq(impactEvaluator.participantCountReadyForTransfer(), 0);
        assertEq(impactEvaluator.participantCountScheduledForTransfer(), 0);
        for (uint i = 0; i < 10; i++) {
            vm.expectRevert();
            impactEvaluator.readyForTransfer(i);
            vm.expectRevert();
            impactEvaluator.scheduledForTransfer(i);
        }
        for (uint i = 0; i < 5; i++) {
            assertEq(
                vm.addr(i + 1).balance,
                10 ether,
                string.concat(
                    "paid participant ",
                    Strings.toString(i),
                    " in second tick"
                )
            );
            assertEq(
                vm.addr(i + 1 + 5).balance,
                10 ether,
                string.concat(
                    "didn't pay participant ",
                    Strings.toString(i),
                    " extra in second tick"
                )
            );
        }

        impactEvaluator.revokeRole(
            impactEvaluator.DEFAULT_ADMIN_ROLE(),
            address(this)
        );
        vm.expectRevert("Not an admin");
        impactEvaluator.setMaxTransfersPerTx(0);
        assertEq(impactEvaluator.maxTransfersPerTx(), 5);
    }

    function test_AvailableBalance() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        assertEq(impactEvaluator.availableBalance(), 0);
        vm.deal(payable(address(impactEvaluator)), 200 ether);
        assertEq(impactEvaluator.availableBalance(), 200 ether);

        impactEvaluator.adminAdvanceRound();
        impactEvaluator.adminAdvanceRound();
        address payable[] memory addresses = new address payable[](1);
        addresses[0] = payable(vm.addr(1));
        uint[] memory scores = new uint[](1);
        scores[0] = impactEvaluator.MAX_SCORE();
        impactEvaluator.setScores(1, addresses, scores);
        assertEq(impactEvaluator.availableBalance(), 100 ether);

        impactEvaluator.adminAdvanceRound();
        addresses[0] = payable(0x000000000000000000000000000000000000dEaD);
        impactEvaluator.setScores(2, addresses, scores);
        assertEq(impactEvaluator.availableBalance(), 100 ether);

        impactEvaluator.releaseRewards();
        assertEq(impactEvaluator.availableBalance(), 100 ether);
    }

    function test_MinBalanceForTransfer() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        vm.deal(payable(address(impactEvaluator)), 100 ether);
        impactEvaluator.setRoundReward(0.4 ether);

        impactEvaluator.adminAdvanceRound();
        impactEvaluator.adminAdvanceRound();

        address payable[] memory addresses = new address payable[](1);
        addresses[0] = payable(vm.addr(1));
        uint[] memory scores = new uint[](1);
        scores[0] = impactEvaluator.MAX_SCORE();
        impactEvaluator.setScores(1, addresses, scores);

        impactEvaluator.releaseRewards();
        assertEq(vm.addr(1).balance, 0);

        impactEvaluator.adminAdvanceRound();
        impactEvaluator.setScores(2, addresses, scores);
        impactEvaluator.releaseRewards();
        assertEq(vm.addr(1).balance, 0.8 ether);

        assertEq(impactEvaluator.minBalanceForTransfer(), 0.5 ether);
    }

    function test_GasSetScores() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        impactEvaluator.adminAdvanceRound();
        impactEvaluator.adminAdvanceRound();

        address payable[] memory addresses = new address payable[](1);
        addresses[0] = payable(vm.addr(1));
        uint[] memory scores = new uint[](1);
        scores[0] = 1;

        for (uint i = 0; i < 1000; i++) {
            impactEvaluator.setScores(1, addresses, scores);
        }
    }

    function test_GasSetScoresManyParticipants() public {
        ImpactEvaluator impactEvaluator = new ImpactEvaluator(address(this));
        impactEvaluator.adminAdvanceRound();
        impactEvaluator.adminAdvanceRound();

        uint participants = 1000;
        address payable[] memory addresses = new address payable[](participants);
        uint[] memory scores = new uint[](participants);
        for (uint i = 0; i < participants; i++) {
            addresses[i] = payable(vm.addr(i + 1));
            scores[i] = 1;
        }

        for (uint i = 0; i < 100; i++) {
            impactEvaluator.setScores(1, addresses, scores);
        }
    }
}
