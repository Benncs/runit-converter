// SPDX-License-Identifier: GPL-3.0-or-later
use dioxus::prelude::*;

#[component]
pub fn NavLateral() -> Element {
    rsx! {

      div{
        id:"lat-1",
        h1{"Options:"}

        div {
          id:"opt-seps",
          h2{"Symbol/Separator"}
          label{"Between units"}
          label{"Exponential symbol"}
        }

      }
    }
}
