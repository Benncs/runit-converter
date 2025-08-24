// SPDX-License-Identifier: GPL-3.0-or-later
mod components;
use components::{Body, Footer, Header};
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    dioxus::launch(App);
}

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(Wrapper)]
        #[route("/")]
        Body{}
}

#[component]
fn Wrapper() -> Element {
    let full_route = use_route::<Route>();
    rsx! {
        header {Header{}}
        // The index route will be rendered here
        Outlet::<Route> {}
        footer { Footer{} }
    }
}
use futures::executor::block_on;
use libunits_converter::{
    InlineUnitParser, MainConverter, MainUnitFactory, construct_all, unitquery::SqlUnitQuery,
};

struct MainContext(
    InlineUnitParser,
    MainUnitFactory<SqlUnitQuery>,
    MainConverter<SqlUnitQuery>,
);

impl MainContext {
    pub fn new() -> Self {
        let (parser, factory, converter) = block_on(construct_all());
        Self(parser, factory, converter)
    }
}

#[component]
fn App() -> Element {
    let global_cxt = use_context_provider(|| Signal::new(MainContext::new()));

    rsx! {
    document::Link { rel: "icon", href: FAVICON }
    document::Link { rel: "stylesheet", href: MAIN_CSS }

    Router::<Route> {} }
}
