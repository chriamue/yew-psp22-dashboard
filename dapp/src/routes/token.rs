use crate::services::polkadot::contracts::calls::types::Call;
use crate::services::polkadot::runtime_types::sp_weights::weight_v2::Weight;
use crate::services::{get_balance, get_total_supply};
use anyhow::anyhow;
use blake2::{Blake2s256, Digest};
use futures::FutureExt;
use std::hash::Hash;
use std::str::FromStr;
use subxt::dynamic::Value;
use subxt::ext::codec::{Decode, Encode};
use subxt::tx::Payload;
use subxt::utils::AccountId32;
use subxt::utils::MultiAddress;
use subxt::utils::MultiSignature;
use subxt::OnlineClient;
use subxt::PolkadotConfig;
use subxt_signer::sr25519::dev;
use wasm_bindgen::JsCast;
use web_sys::EventTarget;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::services::{
    extension_signature_for_partial_extrinsic, get_accounts, polkadot, Account, TokenService,
};

fn blake2_256(input: &[u8]) -> Vec<u8> {
    let mut hasher = Blake2s256::new();
    hasher.update(input);
    hasher.finalize().to_vec()
}

const PROOF_SIZE: u64 = u64::MAX / 2;

fn create_playload() -> Payload<Call> {
    let alice_pair_signer = dev::alice();

    let contract = "5FmHL1qCfDPQMjzbP9wwXFvdF2GKPP8TqZHLQv5UeuAd6gLn";
    let selector = "9bae9d5e";

    let contract: MultiAddress<AccountId32, ()> = AccountId32::from_str(contract).unwrap().into();

    let mut call_data = Vec::<u8>::new();

    let bytes = hex::decode(selector).expect("Decoding failed");

    // Append to the vector
    call_data.extend(bytes);

    web_sys::console::log_1(&format!("Contract: {:?}", contract).into());
    web_sys::console::log_1(&format!("Call Data: {:?}", call_data).into());

    //call_data.append(&mut (&blake2_256("PSP22::total_supply".as_bytes())[0..4]).to_vec());
    /*call_data.append(&mut scale::Encode::encode(
        &(AccountId32::from_str(
            &"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        )
        .unwrap()),
    ));*/
    //let call_data = ("psp22::totalSupply".hash()[..3], ).encode();

    polkadot::tx().contracts().call(
        contract,
        0,
        Weight {
            ref_time: 500_000_000_000,
            proof_size: PROOF_SIZE,
        },
        None,
        call_data,
    )
}

pub struct TokenComponent {
    contract: String,
    account: Option<String>,
    balance: Option<u128>,
    online_client: Option<OnlineClient<PolkadotConfig>>,
    stage: TokenStage,
    token_service: Option<TokenService>,
}

impl TokenComponent {
    /// # Panics
    /// panics if self.online_client is None.
    fn set_contract(&mut self, contract: String) {
        self.contract = contract;
    }
}

pub enum TokenStage {
    Error(String),
    CreatingOnlineClient,
    EnterContract,
    EnterAccount,
    RequestingBalance,
    DisplayBalance(u128), // The balance can be u128 for illustrative purposes
    RequestingAccounts,
    SelectAccount(Vec<Account>),
    Signing(Account),
}

pub enum Message {
    Error(anyhow::Error),
    OnlineClientCreated(OnlineClient<PolkadotConfig>),
    ChangeContract(String),
    TokenServiceCreated(TokenService),
    RequestAccounts,
    ReceivedAccounts(Vec<Account>),
    SignWithAccount(usize),
    RequestBalance,
    ReceivedBalance(u128),
}

impl Component for TokenComponent {
    type Message = Message;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(OnlineClient::<PolkadotConfig>::new().map(|res| {
            match res {
                Ok(online_client) => Message::OnlineClientCreated(online_client),
                Err(err) => Message::Error(anyhow!("Online Client could not be created. Make sure you have a local node running:\n{err}")),
            }
        }));
        ctx.link()
            .send_future(TokenService::new().map(|res| match res {
                Ok(service) => Message::TokenServiceCreated(service),
                Err(err) => Message::Error(anyhow!("Failed to create TokenService: {}", err)),
            }));
        TokenComponent {
            account: None,
            balance: None,
            contract: "5FbxgE9CZgib7p4oWi34Tx5vqLHsXKNGEWnfMn6pMT7VzwTx".to_string(),
            stage: TokenStage::EnterContract,
            online_client: None,
            token_service: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::OnlineClientCreated(online_client) => {
                self.online_client = Some(online_client);
                self.stage = TokenStage::EnterAccount;
            }
            Message::ChangeContract(contract) => {
                self.set_contract(contract);
            }
            Message::TokenServiceCreated(service) => {
                self.token_service = Some(service);
            }
            Message::SignWithAccount(i) => {
                if let TokenStage::SelectAccount(accounts) = &self.stage {
                    let account = accounts.get(i).unwrap();
                    let account_address = account.address.clone();
                    let account_source = account.source.clone();
                    let account_id: AccountId32 = account_address.parse().unwrap();

                    let message = account_address.clone().into_bytes();
                    self.stage = TokenStage::Signing(account.clone());

                    let remark_call = polkadot::tx().system().remark(message.to_vec());

                    //let payload = create_playload();

                    let api = self.online_client.as_ref().unwrap().clone();

                    self.account = Some(account_address.to_string());
                    let contract = self.contract.clone();

                    ctx.link().send_future(async move {
                        get_total_supply(
                            contract,
                        )
                        .await
                        .unwrap();
                        /*
                        web_sys::console::log_1(&format!("Payload: {:?}", payload).into());

                        let result =  api.tx()
                        .sign_and_submit_then_watch_default(&payload, &dev::alice())
                        .await.unwrap().wait_for_finalized_success()
                        .await.unwrap();
                        web_sys::console::log_1(&format!("Run Result: {:?}", result).into());

                        let partial_extrinsic =
                            match api.tx().create_partial_signed(&payload, &account_id, Default::default()).await {
                                Ok(partial_extrinsic) => partial_extrinsic,
                                Err(err) => {
                                    return Message::Error(anyhow!("could not create partial extrinsic:\n{:?}", err));
                                }
                            };

                        let Ok(signature) = extension_signature_for_partial_extrinsic(&partial_extrinsic, &api, &account_id, account_source, account_address).await else {
                            return Message::Error(anyhow!("Signing via extension failed"));
                        };

                        let Ok(multi_signature) = MultiSignature::decode(&mut &signature[..]) else {
                            return Message::Error(anyhow!("MultiSignature Decoding"));
                        };

                        let signed_extrinsic = partial_extrinsic.sign_with_address_and_signature(&account_id.into(), &multi_signature);

                        // do a dry run (to debug in the js console if the extrinsic would work)
                        let dry_res = signed_extrinsic.dry_run(None).await;
                        web_sys::console::log_1(&format!("Dry Run Result: {:?}", dry_res).into());
                        */
                        Message::RequestBalance
                    });
                }
            }
            Message::RequestBalance => {
                if let Some(account) = &self.account {
                    let account_clone = account.clone();
                    let service_clone = self.token_service.as_ref().unwrap().clone();
                    let link = ctx.link();
                    let contract = self.contract.clone();

                    link.send_future(async move {
                        let account_id: AccountId32 =
                            AccountId32::from_str(&account_clone).unwrap();
                        web_sys::console::log_1(&format!("Account ID: {:?}", account_id).into());

                        match get_balance(
                            contract,
                            account_clone.clone(),
                        )
                        .await
                        {
                            Ok(balance) => {
                                web_sys::console::log_1(&format!("Balance: {:?}", balance).into());
                                Message::ReceivedBalance(balance.parse().unwrap())
                            }
                            Err(_) => {
                                Message::Error(anyhow!("Failed to fetch balance.".to_string()))
                            }
                        }
                    });
                }
            }
            Message::ReceivedBalance(balance) => {
                self.balance = Some(balance);
                self.stage = TokenStage::DisplayBalance(balance);
            }
            Message::Error(err) => self.stage = TokenStage::Error(err.to_string()),
            Message::RequestAccounts => {
                self.stage = TokenStage::RequestingAccounts;
                ctx.link().send_future(get_accounts().map(
                    |accounts_or_err| match accounts_or_err {
                        Ok(accounts) => Message::ReceivedAccounts(accounts),
                        Err(err) => Message::Error(err),
                    },
                ));
            }
            Message::ReceivedAccounts(accounts) => {
                self.stage = TokenStage::SelectAccount(accounts);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let contract_html: Html = match &self.stage {
            TokenStage::Error(_)
            | TokenStage::EnterContract
            | TokenStage::CreatingOnlineClient
            | TokenStage::EnterAccount => html!(<></>),
            _ => {
                let on_input = ctx.link().callback(move |event: InputEvent| {
                    let input_element = event.target_dyn_into::<HtmlInputElement>().unwrap();
                    let value = input_element.value();
                    Message::ChangeContract(value)
                });

                html!(
                    <>
                        <div class="mb"><b>{"Enter Contract:"}</b></div>
                        <input oninput={on_input} class="mb" value={AttrValue::from(self.contract.clone())}/>
                    </>
                )
            }
        };
        let stage_html: Html = match &self.stage {
            TokenStage::Error(error_message) => {
                html!(<div class="error"> {"Error: "} {error_message} </div>)
            }
            TokenStage::CreatingOnlineClient => {
                html!(<div>{"Creating Online Client..."}</div>)
            }
            TokenStage::EnterContract => {
                let on_input = ctx.link().callback(move |event: InputEvent| {
                    let input_element = event.target_dyn_into::<HtmlInputElement>().unwrap();
                    let value = input_element.value();
                    Message::ChangeContract(value)
                });

                html!(
                    <>
                        <div class="mb"><b>{"Enter Contract:"}</b></div>
                        <input oninput={on_input} class="mb" value={AttrValue::from(self.contract.clone())}/>
                    </>
                )
            }
            TokenStage::EnterAccount => {
                let get_accounts_click = ctx.link().callback(|_| Message::RequestAccounts);
                html!(<>
                    <div>
                        <button onclick={get_accounts_click}> {"=> Select an Account"} </button>
                    </div>
                </>)
            }
            TokenStage::RequestingAccounts => {
                html!(<div>{"Querying extensions for accounts..."}</div>)
            }
            TokenStage::SelectAccount(accounts) => {
                if accounts.is_empty() {
                    html!(<div>{"No Web3 extension accounts found. Install Talisman or the Polkadot.js extension and add an account."}</div>)
                } else {
                    html!(
                        <>
                            <div class="mb"><b>{"Select an account you want to use for signing:"}</b></div>
                            { for accounts.iter().enumerate().map(|(i, account)| {
                                let sign_with_account = ctx.link().callback(move |_| Message::SignWithAccount(i));
                                html! {
                                    <button onclick={sign_with_account}>
                                        {&account.source} {" | "} {&account.name}<br/>
                                        <small>{&account.address}</small>
                                    </button>
                                }
                            }) }
                        </>
                    )
                }
            }
            TokenStage::Signing(_) => {
                html!(<div>{"Singing message with browser extension..."}</div>)
            }
            TokenStage::RequestingBalance => {
                html!(<div>{"Requesting balance for the account..."}</div>)
            }
            TokenStage::DisplayBalance(balance) => {
                html!(<div>{"Balance: "} {balance} </div>)
            }
        };

        html! {
            <div>
                <a href="/"> <button>{"<= Back"}</button></a>
                <h1>{"Token Management"}</h1>
                {contract_html}
                {stage_html}
            </div>
        }
    }
}
