// SPDX-License-Identifier: (MIT or Apache-2.0)

pragma solidity ^0.8.19;

contract Balances {
    mapping(address => uint) public balances;
    uint public balanceHeld = 0;
    address payable[] public readyForTransfer;
    address payable[] public scheduledForTransfer;
    uint public maxTransfersPerTx = 10;
    uint public minBalanceForTransfer = 0.5 ether;

    event Transfer(address indexed to, uint256 amount);
    event TransferFailed(address indexed to, uint256 amount);

    function rewardsScheduledFor(
        address participant
    ) public view returns (uint) {
        return balances[participant];
    }

    function availableBalance() public view returns (uint) {
        return address(this).balance - balanceHeld;
    }

    function participantCountReadyForTransfer() public view returns (uint) {
        return readyForTransfer.length;
    }

    function participantCountScheduledForTransfer() public view returns (uint) {
        return scheduledForTransfer.length;
    }

    function increaseParticipantBalance(
        address payable participant,
        uint amount
    ) internal {
        uint oldBalance = balances[participant];
        uint newBalance = oldBalance + amount;
        balances[participant] = newBalance;
        balanceHeld += amount;
        if (
            oldBalance <= minBalanceForTransfer &&
            newBalance > minBalanceForTransfer
        ) {
            readyForTransfer.push(participant);
        }
    }

    function _releaseRewards() internal {
        require(
            scheduledForTransfer.length == 0,
            "Scheduled transfers still pending"
        );
        scheduledForTransfer = readyForTransfer;
        delete readyForTransfer;
    }

    function transferScheduled() internal {
        uint totalScheduledForTransfer = scheduledForTransfer.length;
        for (
            uint i = 0;
            i < totalScheduledForTransfer && i < maxTransfersPerTx;
            i++
        ) {
            address payable participant = scheduledForTransfer[
                scheduledForTransfer.length - 1
            ];
            scheduledForTransfer.pop();

            uint amount = balances[participant];
            delete balances[participant];
            balanceHeld -= amount;

            if (participant.send(amount)) {
                emit Transfer(participant, amount);
            } else {
                emit TransferFailed(participant, amount);
            }
        }
    }

    function _setMaxTransfersPerTx(uint _maxTransfersPerTx) internal {
        maxTransfersPerTx = _maxTransfersPerTx;
    }
}
