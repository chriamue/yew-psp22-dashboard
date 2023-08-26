use crate::services::{js_fetch_balance, js_fetch_total_supply};
use anyhow::anyhow;
use subxt::utils::AccountId32;
use subxt::{OnlineClient, PolkadotConfig};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use js_sys::Promise;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = getAccounts)]
    pub fn js_get_accounts() -> Promise;
    #[wasm_bindgen(js_name = signPayload)]
    pub fn js_sign_payload(payload: String, source: String, address: String) -> Promise;
}

#[derive(Clone)]
pub struct TokenService {
    client: OnlineClient<PolkadotConfig>,
}

impl TokenService {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let client = OnlineClient::<PolkadotConfig>::new().await?;

        Ok(TokenService { client })
    }

    pub async fn get_balance_of(
        &self,
        account_id: &AccountId32,
        account_address: String,
    ) -> Result<u128, Box<dyn std::error::Error>> {
        /*
                // Construct the message for the contract call
                let msg = build_message::<FlyconomyTokenRef>(self.contract_address.clone())
                    .call(|contract| contract.balance_of(account_id));

                let result = self.client
                    .call_dry_run(&sender_id, &msg, 0, None)
                    .await
                    .return_value();
        */
        let result = 42;

        // Assuming result can be converted into u128
        Ok(result)
    }

    pub async fn get_balance(&self, contract: String, account: String) -> Result<String, anyhow::Error> {
        let result = JsFuture::from(js_fetch_balance(contract, account))
            .await
            .map_err(|js_err| anyhow!("{js_err:?}"))?;
        let balance = result
            .as_string()
            .ok_or(anyhow!("Error converting JsValue into String"))?;
        Ok(balance)
    }

    pub async fn get_total_supply(&self, contract: String) -> Result<String, anyhow::Error> {
        let result = JsFuture::from(js_fetch_total_supply(contract))
            .await
            .map_err(|js_err| anyhow!("{js_err:?}"))?;
        let total_supply = result
            .as_string()
            .ok_or(anyhow!("Error converting JsValue into String"))?;
        Ok(total_supply)
    }
}
