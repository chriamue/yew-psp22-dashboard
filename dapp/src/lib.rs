use routes::signing::SigningExamplesComponent;
use yew::prelude::*;
use yew_router::prelude::*;

use routes::fetching::FetchingExamplesComponent;
use routes::token::TokenComponent;
mod routes;
mod services;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/fetching")]
    Fetching,
    #[at("/signing")]
    Signing,
    #[at("/token")]
    Token,
    #[not_found]
    #[at("/")]
    Home,
}

pub struct SubxtExamplesApp;

impl Component for SubxtExamplesApp {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        SubxtExamplesApp
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        }
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Fetching => {
            html! { <FetchingExamplesComponent/> }
        }
        Route::Signing => html! { <SigningExamplesComponent/> },
        Route::Token => html! { <TokenComponent/> },
        Route::Home => {
            html! {
            <div>
                <h1>{"Welcome to the Subxt WASM examples!"}</h1>
                <Link<Route> to={Route::Signing}> <button>{"Signing Examples"} </button></Link<Route>>
                <Link<Route> to={Route::Fetching}> <button>{"Fetching and Subscribing Examples"}</button></Link<Route>>
                <Link<Route> to={Route::Token}> <button>{"Token Examples"} </button></Link<Route>>
            </div> }
        }
    }
}
