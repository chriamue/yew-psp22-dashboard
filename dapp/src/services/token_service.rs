use crate::services::polkadot;
use crate::services::polkadot::runtime_types::sp_weights::weight_v2::Weight;
use anyhow::anyhow;
use js_sys::Promise;
use std::str::FromStr;
use subxt::utils::AccountId32;
use subxt::utils::MultiAddress;
use subxt::{OnlineClient, PolkadotConfig};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::JsValue;
use js_sys::Array;

const PROOF_SIZE: u64 = u64::MAX / 2;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = fetchTotalSupply)]
    pub fn js_fetch_total_supply(contract: String) -> Promise;
    #[wasm_bindgen(js_name = fetchBalance)]
    pub fn js_fetch_balance(contract: String, account: String) -> Promise;

    #[wasm_bindgen(js_name = executeContractFunction)]
    pub fn js_execute_contract_function(
        contract: String,
        source: String,
        sender_address: String,
        function_name: String,
        destination_address: String,
        amount: u64,
        args: JsValue
    ) -> Promise;

    #[wasm_bindgen(js_name = queryContract)]
    pub fn js_query_contract(
        contract: String,
        query_function: String,
        args: JsValue
    ) -> Promise;
}

#[derive(Clone)]
pub struct TokenService {
    pub client: OnlineClient<PolkadotConfig>,
}

impl TokenService {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let client = OnlineClient::<PolkadotConfig>::new().await?;

        Ok(TokenService { client })
    }

    pub async fn get_balance(
        &self,
        contract: String,
        account: String,
    ) -> Result<String, anyhow::Error> {
        let args = vec![account.clone()];
        let js_args: Array = args.iter().map(JsValue::from).collect();
        let result = JsFuture::from(js_query_contract(
            contract.clone(),
            "psp22::balanceOf".into(),
            (js_args).into(),
        ))
        .await
        .map_err(|js_err| anyhow!("{js_err:?}"))?;
        println!("result: {:?}", result);
        let balance = result
            .as_string()
            .ok_or(anyhow!("Error converting JsValue into String"))?;
        Ok(balance)
    }

    pub async fn get_total_supply(&self, contract: String) -> Result<String, anyhow::Error> {
        let args: Vec<String> = vec![];
        let js_args: Array = args.iter().map(JsValue::from).collect();
        let result = JsFuture::from(js_query_contract(
            contract,
            "psp22::totalSupply".into(),
            (js_args).into(),
        )).await
            .map_err(|js_err| anyhow!("{js_err:?}"))?;
        let total_supply = result
            .as_string()
            .ok_or(anyhow!("Error converting JsValue into String"))?;
        Ok(total_supply)
    }

    pub async fn transfer_tokens(
        &self,
        contract: String,
        source: String,
        sender_address: String,
        destination_address: String,
        amount: u128,
    ) -> Result<String, anyhow::Error> {
        let args: Vec<String> = vec![];
        let js_args: Array = args.iter().map(JsValue::from).collect();
        let result = JsFuture::from(js_execute_contract_function(
            contract,
            source,
            sender_address,
            "psp22::transfer".into(),
            destination_address,
            amount.try_into().unwrap(),
            (js_args).into(),
        ))
        .await
        .map_err(|js_err| anyhow!("{js_err:?}"))?;
        let result = result
            .as_string()
            .ok_or(anyhow!("Error converting JsValue into String"))?;
        Ok(result)
    }

    pub async fn get_total_supply_native(&self, contract: String) -> Result<u128, anyhow::Error> {
        let result = 42;
        let mut selector: Vec<u8> = [0x16, 0x2d, 0xf8, 0xc2].into();
        let mut data = Vec::new();
        data.append(&mut selector);

        let contract: MultiAddress<AccountId32, ()> =
            AccountId32::from_str(&contract).unwrap().into();
        let signer = subxt_signer::sr25519::dev::alice();
        let payload = polkadot::tx().contracts().call(
            contract, // contract address
            0,        // transferred_value
            Weight {
                ref_time: 11_344_007_255,
                proof_size: 110307,
            },
            None, // storage deposit limit
            data, // the call data
        );

        let trinsic = self.client.tx().create_unsigned(&payload)?;

        let validate = self.client.tx().validate(&payload);
        assert!(validate.is_ok());

        let events = self
            .client
            .tx()
            .sign_and_submit_then_watch_default(&payload, &signer)
            .await?
            .extrinsic_hash();

        web_sys::console::log_1(&format!("tx finalized: {:?}", events).into());
        println!("tx finalized: {:?}", events);

        Ok(42)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[wasm_bindgen_test::wasm_bindgen_test]
    async fn test_get_total_supply() -> Result<(), Box<dyn std::error::Error>> {
        let token_service = TokenService::new().await.unwrap();
        let contract = "5FbxgE9CZgib7p4oWi34Tx5vqLHsXKNGEWnfMn6pMT7VzwTx".to_string();

        let total_supply = token_service
            .get_total_supply_native(contract)
            .await
            .unwrap();
        assert_eq!(total_supply, 42);

        Ok(())
    }
}
