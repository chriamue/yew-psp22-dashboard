use anyhow::anyhow;
use futures::FutureExt;
use std::str::FromStr;
use subxt::utils::AccountId32;
use subxt::{OnlineClient, PolkadotConfig};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::components::SendTokenComponent;
use crate::services::{get_accounts, Account, TokenService};
use crate::Route;

pub struct TokenComponent {
    contract: String,
    account: Option<Account>,
    balance: Option<u128>,
    online_client: Option<OnlineClient<PolkadotConfig>>,
    stage: TokenStage,
    token_service: Option<TokenService>,
}

impl TokenComponent {
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
    SendToken(String, u128),
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
                    self.account = Some(account.clone());
                    let contract = self.contract.clone();
                    let token_service = self.token_service.clone().unwrap();

                    ctx.link().send_future(async move {
                        token_service.get_total_supply(contract).await.unwrap();
                        Message::RequestBalance
                    });
                }
            }
            Message::RequestBalance => {
                if let Some(account) = &self.account {
                    let account_clone = account.address.clone();
                    let link = ctx.link();
                    let contract = self.contract.clone();

                    let token_service = self.token_service.clone().unwrap();

                    link.send_future(async move {
                        let account_id: AccountId32 =
                            AccountId32::from_str(&account_clone).unwrap();
                        web_sys::console::log_1(&format!("Account ID: {:?}", account_id).into());

                        match token_service
                            .get_balance(contract, account_clone.clone())
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
            Message::SendToken(to_address, amount) => {
                let account = self.account.clone().unwrap();
                let account_source = account.source.clone();
                let account_address = account.address.clone();
                let contract = self.contract.clone();
                let token_service = self.token_service.clone().unwrap();

                ctx.link().send_future(async move {
                    match token_service
                        .transfer_tokens(
                            contract,
                            account_source,
                            account.address,
                            to_address,
                            amount,
                        )
                        .await
                    {
                        Ok(result) => {
                            web_sys::console::log_1(&format!("Result: {:?}", result).into());
                            Message::RequestBalance
                        }
                        Err(err) => Message::Error(err),
                    }
                });
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
        let send_token_html: Html = match &self.stage {
            TokenStage::RequestingBalance | TokenStage::DisplayBalance(_) => {
                let send_callback = ctx
                    .link()
                    .callback(|(to_address, amount)| Message::SendToken(to_address, amount));
                html!(
                    <>
                        <SendTokenComponent onsend = {send_callback}/>
                    </>
                )
            }
            TokenStage::Error(_) | _ => html!(<></>),
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
                <Link<Route> to={Route::Home}> <button>{"<= Back"}</button></Link<Route>>
                <h1>{"Token Management"}</h1>
                {contract_html}
                {stage_html}
                {send_token_html}
            </div>
        }
    }
}
