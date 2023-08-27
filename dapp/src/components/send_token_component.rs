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

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub onsend: Callback<(String, u128)>,
}

impl Component for SendTokenComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        SendTokenComponent {
            to_address: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
            amount: 1,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
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
                ctx.props()
                    .onsend
                    .emit((self.to_address.clone(), self.amount));
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;
    use wasm_bindgen::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test::wasm_bindgen_test]
    async fn create_default() {
        let onsend = Callback::default();
        let rendered = yew::LocalServerRenderer::<SendTokenComponent>::with_props(Props { onsend })
            .render()
            .await;

        assert!(rendered.contains("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"));
        assert!(rendered.contains("1"));
    }

    #[wasm_bindgen_test::wasm_bindgen_test]
    async fn update_address() {
        let onsend = Callback::default();
        let rendered = yew::LocalServerRenderer::<SendTokenComponent>::with_props(Props { onsend })
            .render()
            .await;

        assert!(rendered.contains("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"));
    }
}
