// SPDX-License-Identifier: (MIT or Apache-2.0)

import "../lib/openzeppelin-contracts/contracts/access/AccessControl.sol";
pragma solidity ^0.8.19;

/**
 * @title Measure Contract
 * @dev This contract submits commitments of arbitrary data on chain.
 */
contract Measure is AccessControl {
    event Measurement(string dataCommitment);
    bytes32 public constant COMMITMENT_ROLE = keccak256("COMMITMENT_ROLE");

    /**
     * @dev Creates a measure contract with an admin
     * @param admin The address of the factory admin.
     **/
    constructor(address admin) {
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(COMMITMENT_ROLE, admin);
    }

    /**
     * @dev Emmits a Measurement event with a commitment to some arbitrary data.
     * @param dataCommitment An arbitrary commitment to some data (eg. Merkle Root, KZG Commitment, etc.)
     **/
    function measure(string memory dataCommitment) public {
        require(hasRole(COMMITMENT_ROLE, msg.sender));
        emit Measurement(dataCommitment);
    }
}
