use std::str::FromStr;

use ink::primitives::AccountId;
use polkadot::runtime_types::sp_weights::weight_v2::Weight;
use sp_core::{bytes::from_hex, Bytes};
use subxt::{
    utils::{AccountId32, MultiAddress},
    OnlineClient, PolkadotConfig,
};
use subxt_signer::sr25519::dev;

#[subxt::subxt(runtime_metadata_path = "examples/metadata.scale")]
pub mod polkadot {}

const REF_TIME: u64 = 9_375_000_000;
const PROOF_SIZE: u64 = 524_288;

const SMART_CONTRACT: &str = "5DcaqqqKmkcCPYuN9TnthnGDraEAi63SDn9y5qzSA2VETKym";
const SELECTOR: &'static str = "0x162df8c2";
const METHOD: &str = "psp22::totalSupply";

fn address_from_string(address: String) -> AccountId {
    let address: AccountId32 = AccountId32::from_str(&address).unwrap();
    address.0.into()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new API client, configured to talk to Polkadot nodes.
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    let alice_pair_signer = dev::alice();

    let contract: MultiAddress<AccountId32, ()> =
        AccountId32::from_str(SMART_CONTRACT).unwrap().into();

    let mut call_data = Vec::<u8>::new();

    //let bytes = hex::decode(SELECTOR).expect("Decoding failed");

    // Append to the vector
    //call_data.extend(bytes);

    let call_data = from_hex(SELECTOR).unwrap();

    //let tx = subxt::dynamic::tx("Contracts", "call", call_data.clone()).unwrap();

    let call = polkadot::tx().contracts().call(
        contract.clone(),
        0,
        Weight {
            ref_time: REF_TIME,
            proof_size: PROOF_SIZE,
        },
        None,
        call_data,
    );

    let tx_progress = api
        .tx()
        .sign_and_submit_then_watch_default(&call, &alice_pair_signer)
        .await?;

    //println!("Tx Hash: {:?}", tx_progress);

    let result = tx_progress
    .wait_for_in_block()
    .await
    .unwrap_or_else(|err| {
        panic!("error on call `wait_for_in_block`: {err:?}");
    })
    .fetch_events()
    .await
    .unwrap_or_else(|err| {
        panic!("error on call `fetch_events`: {err:?}");
    });

    println!("Result: {:?}", result);

    println!("Call: {:?}", call);
    let result = api
        .tx()
        .sign_and_submit_then_watch_default(&call, &alice_pair_signer)
        .await
        .map(|e| {
            println!("Collection creation submitted, waiting for transaction to be finalized...");
            e
        })?;
    //println!("Tx Hash: {:?}", result);
    let result = result.wait_for_finalized_success().await;
    match result {
        Ok(result) => println!("Result: {:?}", result),
        Err(e) => println!("Error: {:?}", e),
    }

    Ok(())
}