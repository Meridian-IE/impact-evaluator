// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/access/AccessControl.sol";

/**
 * @title MultiTransfer
 * @dev This contract allows the admin to send ETH to multiple recipients in a single transaction.
 *      If a transfer fails, it's skipped.
 *      Failed transfers' amounts are accumulated and sent back to the admin.
 *      Only an admin can initiate transfers and withdraw funds.
 */
contract MultiTransfer is AccessControl {
    /// @notice Emitted when a transfer is successful.
    event Transfer(address indexed to, uint256 amount);

    /// @notice Emitted when a transfer fails.
    event TransferFailed(address indexed to, uint256 amount);

    /// @notice Emitted when the admin withdraws funds.
    event Withdrawn(address indexed admin, uint256 amount);

    /**
     * @dev Sets the original `msg.sender` as the admin.
     */
    constructor() {
        _setupRole(DEFAULT_ADMIN_ROLE, msg.sender);
    }

    /**
     * @notice Transfers the specified amounts of ETH to the specified recipients.
     * @param recipients Array of recipient addresses.
     * @param amounts Array of amounts of ETH to send.
     * @dev The arrays `recipients` and `amounts` must have the same length.
     *      If a transfer fails, the transfer is skipped.
     *      Failed transfers' amounts are accumulated and sent back to the admin.
     *      Can only be called by the admin.
     */
    function multiTransfer(
        address payable[] memory recipients,
        uint256[] memory amounts
    ) external onlyRole(DEFAULT_ADMIN_ROLE) {
        require(
            recipients.length == amounts.length,
            "Recipients and amounts length mismatch"
        );

        uint256 refundAmount = 0;

        for (uint256 i = 0; i < recipients.length; i++) {
            uint256 amount = amounts[i];
            if (!attemptTransfer(recipients[i], amount)) {
                emit TransferFailed(recipients[i], amount);
                refundAmount += amount;
            }
        }

        if (refundAmount > 0) {
            bool sent = payable(msg.sender).send(refundAmount);
            require(sent, "Refund failed");
        }
    }

    /**
     * @notice Tries to transfer ETH to the recipient.
     * @param recipient Address of the recipient.
     * @param amount Amount of ETH to send.
     * @return success True if the transfer was successful, false otherwise.
     * @dev This function uses `.send` to transfer ETH, which is a safer method but can fail for various reasons.
     */
    function attemptTransfer(
        address payable recipient,
        uint256 amount
    ) internal returns (bool) {
        bool success = recipient.send(amount);
        if (success) {
            emit Transfer(recipient, amount);
        }
        return success;
    }

    /**
     * @notice Withdraws all funds from the contract.
     * @dev Only callable by the admin. Emits a `Withdrawn` event on success.
     */
    function withdraw() external onlyRole(DEFAULT_ADMIN_ROLE) {
        uint256 balance = address(this).balance;
        require(balance > 0, "No balance to withdraw");
        bool sent = payable(msg.sender).send(balance);
        require(sent, "Withdrawal failed");
        emit Withdrawn(msg.sender, balance);
    }

    /// @notice Fallback function to allow the contract to accept ETH.
    receive() external payable {}
}
