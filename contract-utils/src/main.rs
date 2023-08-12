use contract_bindings::measure::MEASURE_ABI;
use contract_utils::measure::{deploy_contract, DeployMethod};
use ethers::providers::Middleware;
use ethers::utils::__serde_json::ser;
use fevm_utils::{get_ledger_signing_provider, get_provider, get_wallet_signing_provider};
use std::fs::{read_to_string, write};
use std::sync::Arc;
use std::{path::PathBuf, str::FromStr};

const DEPLOY_METHOD: DeployMethod = DeployMethod::Mnemonic;
const RPC_URL: &str = "https://api.calibration.node.glif.io/rpc/v1";
const SECRETS_PATH: &str = "../secrets/mnemonic";
const ABI_WRITE_PATH: &str = "../MeasureAbi.json";

const RETRIES: usize = 15;

#[tokio::main]
async fn main() {
    let provider = get_provider(&RPC_URL).unwrap();
    let chain_id = provider.get_chainid().await.unwrap();
    let string_abi = ser::to_string(&MEASURE_ABI.clone()).unwrap();
    write(&ABI_WRITE_PATH, string_abi).unwrap();

    match DEPLOY_METHOD {
        DeployMethod::Ledger => {
            let ledger_client = get_ledger_signing_provider(provider.clone(), chain_id.as_u64())
                .await
                .unwrap();
            let client = Arc::new(ledger_client);
            deploy_contract(client.clone(), RETRIES, provider, client.address())
                .await
                .unwrap();
        }
        DeployMethod::Mnemonic => {
            let secret = PathBuf::from_str(SECRETS_PATH).unwrap();
            let mnemonic = read_to_string(secret).unwrap();
            let local_client = get_wallet_signing_provider(provider.clone(), &mnemonic)
                .await
                .unwrap();
            let client = Arc::new(local_client);
            deploy_contract(client.clone(), RETRIES, provider, client.address())
                .await
                .unwrap();
        }
    }
}
