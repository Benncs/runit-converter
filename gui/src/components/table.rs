// SPDX-License-Identifier: GPL-3.0-or-later
use dioxus::prelude::*;
#[component]
pub fn UnitTable() -> Element {
    rsx! {table{
      id:"unit-table",
      tr{
        th {"Value"}
        th{"Unit 1"}
        th{"Unit 2"}
        th{"Result"}
      }
      tr{
        td{"test2"}
        td{"test2"}
        td{"test2"}
        td{"test2"}
      }
    }}
}
