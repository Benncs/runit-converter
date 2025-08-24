// SPDX-License-Identifier: GPL-3.0-or-later
mod lateral;
mod table;
use dioxus::prelude::*;
use lateral::NavLateral;
use libunits_converter::UnitParser;
use table::UnitTable;

use crate::MainContext;
const TABLE_CSS: Asset = asset!("/assets/styling/table.css");

#[component]
pub fn Body() -> Element {
    rsx! {
      document::Link {
          rel: "stylesheet",
          href: TABLE_CSS,
      }
      div {
        id: "app-body",

        NavLateral {  }
        span { class:"vsep" }
        div{
          id:"table-btn",
        UnitTable {  }

        }
    }
    }
}

#[component]
pub fn Header() -> Element {
    rsx! {
      div {
        id: "app-header",

        // img { src: HEADER_SVG, id: "header" }
        h1 {
          "Unit Converter"
        }

      }
    }
}

#[component]
pub fn Footer() -> Element {
    rsx! {
      footer {
        id: "app-footer",
        p{" RUnitConverter Copyright (C) 2025 Casale Benjamin"}
      }
    }
}
