// SPDX-License-Identifier: GPL-3.0-or-later
mod components;
use dioxus::prelude::*;

use components::{Body, Footer, Header};

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

#[component]
fn App() -> Element {
    rsx! {
    document::Link { rel: "icon", href: FAVICON }
    document::Link { rel: "stylesheet", href: MAIN_CSS }

    Router::<Route> {} }
}
