// SPDX-License-Identifier: (MIT or Apache-2.0)

pragma solidity ^0.8.19;

contract Balances {
    mapping(address => uint) public balances;
    uint public balanceHeld = 0;
    address payable[] readyForTransfer;
    address payable[] scheduledForTransfer;
    uint public maxTransfersPerTick = 10;
    uint public minBalanceForTransfer = 1 ether;

    event Transfer(address indexed to, uint256 amount);
    event TransferFailed(address indexed to, uint256 amount);

    function balanceOf(address participant) public view returns (uint) {
        return balances[participant];
    }

    function _setMaxTransfersPerTick(uint _maxTransfersPerTick) internal {
        maxTransfersPerTick = _maxTransfersPerTick;
    }

    function _setMinBalanceForTransfer(uint _minBalanceForTransfer) internal {
        minBalanceForTransfer = _minBalanceForTransfer;
    }

    function reserveBalance(uint amount) internal {
        // `advanceRound` ensures `amount` doesn't exceed the available balance
        balanceHeld += amount;
    }

    function releaseBalance(uint amount) internal {
        balanceHeld -= amount;
    }

    function availableBalance() public view returns (uint) {
        return address(this).balance - balanceHeld;
    }

    function increaseParticipantBalance(
        address payable participant,
        uint amount
    ) internal {
        uint oldBalance = balances[participant];
        uint newBalance = oldBalance + amount;
        balances[participant] = newBalance;
        if (
            oldBalance <= minBalanceForTransfer &&
            newBalance > minBalanceForTransfer
        ) {
            readyForTransfer.push(participant);
        }
    }

    function _releaseRewards() internal {
        scheduledForTransfer = readyForTransfer;
        delete readyForTransfer;
    }

    function maybeTransferRewards() internal {
        for (
            uint i = 0;
            i < scheduledForTransfer.length && i < maxTransfersPerTick;
            i++
        ) {
            address payable participant = scheduledForTransfer[
                scheduledForTransfer.length - 1
            ];
            uint amount = balanceOf(participant);
            if (participant.send(amount)) {
                emit Transfer(participant, amount);
            } else {
                emit TransferFailed(participant, amount);
            }
            scheduledForTransfer.pop();
        }
    }
}
