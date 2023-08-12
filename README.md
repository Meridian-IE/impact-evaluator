# meridian-measure-contract
[Meridian measure contract](https://pl-strflt.notion.site/Meridian-Design-Doc-03-Evaluation-dissected-52803c22ee564e2ab8a86756fffa2693?pvs=4)

## Development

This repo requires Rust and Cargo, which can be installed from [here](https://doc.rust-lang.org/book/ch01-01-installation.html)

##### Install Foundry
We recommend you install it from source:

```bash
git clone https://github.com/foundry-rs/foundry
cd foundry
# install cast + forge
cargo install --path ./cli --profile local --bins --locked --force
# install anvil
cargo install --path ./anvil --profile local --locked --force
```

##### Clone Repo and Install

```bash
git clone https://github.com/meridian-ie/measure-contract.git
cd measure-contract
git submodule update --recursive
forge test
```

##### Local Development

To iterate quickly, the  `anvil` network can be used to develop locally. Note that Anvil strictly uses Etheruem addressing and does not include Filecoin pre-compiles.

First run `anvil`:
```bash
anvil
```
The output will provide a list of private keys and addresses that can be used. Anvil's default mnemonic is:
`test test test test test test test test test test test junk`

To deploy the contract on Anvil, run:

```bash
forge create --rpc-url http://127.0.0.1:8545 --mnemonic "test test test test test test test test test test test junk" src/Measure.sol:Measure --constructor-args 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
```

## Deployment

Contracts can be deployed using the `forge cli` or using a rust deployment script that leverages contract bindings.
### Forge CLI
This deployment method requires manual insertion of a private key and is not recommended for production use cases.

NOTE: Deployment using forge CLI often errors out on Filecoin networks even though the transaction goes through (Foundry
is configured for EVM's block time, not FVM's). Use a block explorer to find the address of the contract.

Make sure the following env vars are defined as follows:
```bash
export RPC_URL="..."
export ADMIN_ADDRESS="..."
export MNEMONIC_PATH="{path to mnemonic secret file}"
```

To deploy using a private key, run:

```bash
forge create --rpc-url $RPC_URL --private-key <your_private_key> src/Measure.sol:Measure --constructor-args $ADMIN_ADDRESS
```

To deploy using a local mnemonic secret, run:
```bash
forge create --rpc-url $RPC_URL --mnemonic $MNEMONIC_PATH src/Measure.sol:Measure --constructor-args $ADMIN_ADDRESS
```


### Deployment Rust Script

The deployment relies on contract bindings generated in the `/contract-bindings` directory. If you make changes to the contracts, run:

```bash
rm -rf contract-bindings
forge bind  --crate-name contract-bindings -b ./contract-bindings
```
This will create new bindings with the modified contracts.

Deployment can then proceed either with a locally stored mnemonic or a connected ethereum ledger wallet. To use with a mnemonic, create a `secrets/mnemonic.txt` file in the root directory.

To deploy, run:
```bash
cd contract-utils
cargo run
```


## Tests

#### Integration Tests

Integration tests run on the filecoin calibration net and require a wallet with test FIL to pay for gas fees on the calibration net. Test FIL is free and can be obtained using the [faucet](https://faucet.calibration.fildev.network/).

Before running integration tests, these env vars are required:

```bash
export TEST_RPC_URL=https://api.calibration.node.glif.io/rpc/v1
export TEST_MNEMONIC={insert wallet mnemonic here}
export TEST_CONTRACT_ADDRESS={this can be an empty string}
```

To run tests, run:

```bash
cd contract-utils
cargo test  -- --nocapture --test-threads 1
```
