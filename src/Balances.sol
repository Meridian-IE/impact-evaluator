// SPDX-License-Identifier: (MIT or Apache-2.0)

import "../lib/openzeppelin-contracts/contracts/utils/cryptography/ECDSA.sol";
pragma solidity ^0.8.19;

contract Balances {
    mapping(address => uint) public balances;
    uint public balanceHeld = 0;

    function balanceOf(address account) public view returns (uint) {
        return balances[account];
    }

    function availableBalance() public view returns (uint) {
        return address(this).balance - balanceHeld;
    }

    function releaseBalance(uint amount) internal {
        balanceHeld -= amount;
    }

    function reserveBalance(uint amount) internal {
        // `advanceRound` ensures `amount` doesn't exceed the available balance
        balanceHeld += amount;
    }

    function assignBalance(address account, uint amount) internal {
        balances[account] += amount;
    }
}
