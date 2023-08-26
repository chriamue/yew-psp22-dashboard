use anyhow::anyhow;
use js_sys::Promise;
use subxt::utils::AccountId32;
use subxt::{OnlineClient, PolkadotConfig};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = fetchTotalSupply)]
    pub fn js_fetch_total_supply(contract: String) -> Promise;
    #[wasm_bindgen(js_name = fetchBalance)]
    pub fn js_fetch_balance(contract: String, account: String) -> Promise;
    #[wasm_bindgen(js_name = transferTokens)]
    pub fn js_transfer_tokens(contract: String, source: String, sender_address: String, destination_address: String, amount: u64) -> Promise;
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

    pub async fn get_balance(
        &self,
        contract: String,
        account: String,
    ) -> Result<String, anyhow::Error> {
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

    pub async fn transfer_tokens(&self, contract: String, source: String, sender_address: String, destination_address: String, amount: u128) -> Result<String, anyhow::Error> {
        let result = JsFuture::from(js_transfer_tokens(contract, source, sender_address, destination_address, amount.try_into().unwrap()))
            .await
            .map_err(|js_err| anyhow!("{js_err:?}"))?;
        let result = result
            .as_string()
            .ok_or(anyhow!("Error converting JsValue into String"))?;
        Ok(result)
    }
}
