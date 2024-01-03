// SPDX-License-Identifier: (MIT or Apache-2.0)

pragma solidity ^0.8.19;

contract Balances {
    mapping(address => uint) public balances;
    uint public maxTransfersPerTx = 10;
    uint public constant minBalanceForTransfer = 0.5 ether;

    event Transfer(address indexed to, uint256 amount);
    event TransferFailed(address indexed to, uint256 amount);

    struct ScheduledTransfer {
        address payable participant;
        uint amount;
    }
    ScheduledTransfer[] public scheduledTransfers;

    function rewardsScheduledFor(
        address participant
    ) public view returns (uint) {
        return balances[participant];
    }

    function participantCountScheduledForTransfer() public view returns (uint) {
        return scheduledTransfers.length;
    }

    function scheduleTransfer(
        address payable participant,
        uint amount
    ) internal {
        scheduledTransfers.push(ScheduledTransfer({
            participant: participant,
            amount: amount
        }));
    }

    function transferScheduled() internal returns (uint) {
        uint removedBalance = 0;
        uint totalScheduledForTransfer = scheduledTransfers.length;
        for (
            uint i = 0;
            i < totalScheduledForTransfer && i < maxTransfersPerTx;
            i++
        ) {
            ScheduledTransfer memory scheduledTransfer = scheduledTransfers[
                scheduledTransfers.length - 1
            ];
            scheduledTransfers.pop();

            if (scheduledTransfer.participant.send(scheduledTransfer.amount)) {
                emit Transfer(
                    scheduledTransfer.participant,
                    scheduledTransfer.amount
                );
            } else {
                emit TransferFailed(
                    scheduledTransfer.participant,
                    scheduledTransfer.amount
                );
            }
        }
    }

    function _setMaxTransfersPerTx(uint _maxTransfersPerTx) internal {
        maxTransfersPerTx = _maxTransfersPerTx;
    }
}
