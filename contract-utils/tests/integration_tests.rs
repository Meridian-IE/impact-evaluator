use assert_fs::fixture::FileWriteStr;
use assert_fs::NamedTempFile;
use contract_utils::measure::deploy_contract;
use once_cell::sync::Lazy;
use std::fs::read_to_string;
use std::sync::Mutex;
use std::{env, sync::Arc};

use fevm_utils::{get_provider, get_wallet_signing_provider};

static RPC_URL: Lazy<String> =
    Lazy::new(|| env::var("TEST_RPC_URL").expect("TEST_RPC_URL must be set"));
static MNEMONIC: Lazy<String> =
    Lazy::new(|| env::var("TEST_MNEMONIC").expect("TEST_MNEMONIC must be set"));

static CONTRACT_ADDRESS: Lazy<Mutex<String>> = Lazy::new(|| {
    Mutex::new(env::var("TEST_CONTRACT_ADDRESS").expect("TEST_CONTRACT_ADDRESS must be set"))
});
static SECRETS_FILE: Lazy<NamedTempFile> = Lazy::new(|| {
    let file = assert_fs::NamedTempFile::new("secrets.txt").unwrap();
    file.write_str(MNEMONIC.as_str()).unwrap();
    file
});

#[tokio::test]
async fn deploy() -> Result<(), Box<dyn std::error::Error>> {
    let provider = get_provider(&RPC_URL).unwrap();
    let mnemonic = read_to_string(SECRETS_FILE.path()).unwrap();
    let local_client = get_wallet_signing_provider(provider.clone(), &mnemonic)
        .await
        .unwrap();
    let client = Arc::new(local_client);
    let address = deploy_contract(client.clone(), 15, provider, client.address())
        .await
        .unwrap();

    let mut data = CONTRACT_ADDRESS.lock().unwrap();
    *data = address.to_string();

    Ok(())
}
