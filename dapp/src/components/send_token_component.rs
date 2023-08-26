use wasm_bindgen::JsCast;
use web_sys::EventTarget;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub struct SendTokenComponent {
    to_address: String,
    amount: u128,
}

pub enum Msg {
    UpdateAddress(String),
    UpdateAmount(String),
    SendTokens,
}

impl Component for SendTokenComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        SendTokenComponent {
            to_address: String::new(),
            amount: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateAddress(addr) => {
                self.to_address = addr;
            }
            Msg::UpdateAmount(amt_str) => {
                if let Ok(amt) = amt_str.parse::<u128>() {
                    self.amount = amt;
                }
            }
            Msg::SendTokens => {
                // Logic to send tokens
                // You can use the logic from your TokenComponent or another service to actually send the tokens here
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let update_address = ctx.link().callback(|e: Event| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");
            let value = target.unchecked_into::<HtmlInputElement>().value();
            Msg::UpdateAddress(value)
        });

        let update_amount = ctx.link().callback(|e: Event| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");
            let value = target.unchecked_into::<HtmlInputElement>().value();
            Msg::UpdateAmount(value)
        });

        let send_tokens = ctx.link().callback(|_| Msg::SendTokens);

        html! {
            <div>
                <div>
                    <input
                        value={self.to_address.clone()}
                        onchange={update_address}
                        placeholder="Address"
                    />
                </div>
                <div>
                    <input
                        value={self.amount.to_string()}
                        onchange={update_amount}
                        placeholder="Amount"
                        type="number"
                    />
                </div>
                <div>
                    <button onclick={send_tokens}>{"Send Tokens"}</button>
                </div>
            </div>
        }
    }
}
