// SPDX-License-Identifier: (MIT or Apache-2.0)

import "../lib/openzeppelin-contracts/contracts/utils/Nonces.sol";
import "../lib/openzeppelin-contracts/contracts/utils/cryptography/ECDSA.sol";
pragma solidity ^0.8.19;

contract Balances is Nonces {
    event Withdrawal(address indexed account, address target, uint256 value);

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

    function _withdraw(
        address account,
        address payable target,
        uint value
    ) private {
        require(balances[account] >= value, "Insufficient balance");
        balances[account] -= value;
        balanceHeld -= value;
        if (balances[account] == 0) {
            delete balances[account];
        }
        require(target.send(value), "Withdrawal failed");
    }

    function withdraw(address payable target, uint value) public {
        _withdraw(msg.sender, target, value);
        emit Withdrawal(msg.sender, target, value);
    }

    function withdrawOnBehalf(
        address account,
        address payable target,
        uint value,
        uint8 v,
        bytes32 r,
        bytes32 s
    ) public {
        bytes32 digest = keccak256(
            abi.encode(account, _useNonce(account), msg.sender, target, value)
        );
        address signer = ECDSA.recover(digest, v, r, s);
        require(signer == account, "Invalid signature");

        _withdraw(account, payable(msg.sender), 0.1 ether);
        _withdraw(account, target, value - 0.1 ether);
        emit Withdrawal(account, target, value - 0.1 ether);
    }
}
