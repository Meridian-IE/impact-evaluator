// SPDX-License-Identifier: (MIT or Apache-2.0)

pragma solidity ^0.8.19;

contract Balances {
    address payable[] participantsReadyForTransfer;
    mapping(address => uint) public balances;
    uint public balanceHeld = 0;
    uint public maxTransfersPerTick = 10;

    event Transfer(address indexed to, uint256 amount);
    event TransferFailed(address indexed to, uint256 amount);

    function balanceOf(address participant) public view returns (uint) {
        return balances[participant];
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
        if (oldBalance <= 1 ether && newBalance > 1 ether) {
            participantsReadyForTransfer.push(participant);
        }
    }

    function maybeTransferRewards() internal {
        if (participantsReadyForTransfer.length > 0) {
            transferRewards();
        }
    }

    function transferRewards() private {
        if (participantsReadyForTransfer.length <= maxTransfersPerTick) {
            for (uint i = 0; i < participantsReadyForTransfer.length; i++) {
                transferReward(i);
            }
            delete participantsReadyForTransfer;
        } else {
            for (
                uint i = 0;
                i < maxTransfersPerTick &&
                    i < participantsReadyForTransfer.length;
                i++
            ) {
                uint index = uint(blockhash(block.number + i)) %
                    participantsReadyForTransfer.length;
                transferReward(index);
                participantsReadyForTransfer[
                    index
                ] = participantsReadyForTransfer[
                    participantsReadyForTransfer.length - 1
                ];
                participantsReadyForTransfer.pop();
            }
        }
    }

    function transferReward(uint index) private {
        address payable participant = participantsReadyForTransfer[index];
        uint amount = balanceOf(participant);

        if (participant.send(amount)) {
            emit Transfer(participant, amount);
        } else {
            emit TransferFailed(participant, amount);
        }
    }
}
