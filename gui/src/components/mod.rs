// SPDX-License-Identifier: GPL-3.0-or-later
mod lateral;
mod table;
use dioxus::prelude::*;
use table::UnitTable;

const HEADER_SVG: Asset = asset!("/assets/header.svg");
#[component]
pub fn Body() -> Element {
    rsx! {
      div {
        id: "app-body",

        UnitTable {  }

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
