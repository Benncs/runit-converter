// SPDX-License-Identifer: GPL-3.0-or-later
use dioxus::prelude::*;
use libunits_converter::{UnitConverter, UnitFactory, UnitParser, Value};

use crate::MainContext;

#[derive(Clone, PartialEq)]
struct UnitState {
    val: Signal<f64>,
    unit1: Signal<String>,
    unit2: Signal<String>,
    res: Signal<f64>,
}

impl UnitState {
    fn new() -> Self {
        UnitState {
            val: use_signal(|| 0.),
            unit1: use_signal(String::new),
            unit2: use_signal(String::new),
            res: use_signal(|| 0.),
        }
    }
    fn get_data(&self) -> (f64, String, String, Signal<f64>) {
        (
            *self.val.read(),
            self.unit1.read().clone(),
            self.unit2.read().clone(),
            self.res,
        )
    }
}

#[component]
fn UnitInput(unit: Signal<String>) -> Element {
    rsx! {
       input {
         value: "{unit}",
         placeholder: "Unit name",
         oninput: move |event| unit.set(event.value()),
       }
    }
}

#[component]
fn ValUnitInput(unit: Signal<f64>) -> Element {
    rsx! {
       input {
         value: "{unit}",
         oninput: move |event| {
             if let Ok(val) = event.value().parse::<f64>() {
                 unit.set(val);
             }
             // else{unit.set(0.)}
         },
       }
    }
}

#[component]
fn RowUnitTable(
    on_remove: EventHandler,
    on_ok: EventHandler<(f64, String, String, Signal<f64>)>,
) -> Element {
    let state = UnitState::new();
    let s = state.clone();

    let ok_click = move |_| {
        on_ok.call(s.get_data());
    };

    rsx! {
        tr {
            td { ValUnitInput { unit: state.val } }
            td { UnitInput { unit: state.unit1 } }
            td { UnitInput { unit: state.unit2 } }
            td { "{state.res.read()}" }
            td {
                button { onclick: move |_| on_remove.call(()), "Delete" }
            }
            td {
                button {
                    id: "convert",
                    onclick: ok_click,
                    "Ok"
                }
            }
        }
    }
}

#[component]
pub fn UnitTable() -> Element {
    let ctx = use_context::<Signal<MainContext>>();
    let mut rows = use_signal(Vec::<UnitState>::new);
    let add_row = move |_| {
        rows.with_mut(|v| v.push(UnitState::new()));
    };
    let mut remove_row = move |idx: usize| {
        rows.with_mut(|v| {
            if idx < v.len() {
                v.remove(idx);
            }
        });
    };

    let handle_ok = move |(val, unit1, unit2, mut res): (f64, String, String, Signal<f64>)| {
        let parser = &ctx.read().0;
        let factory = &ctx.read().1;
        let converter = &ctx.read().2;

        let runit1 = factory.parse_fill(parser, &unit1);
        let runit2 = factory.parse_fill(parser, &unit2);

        if let (Ok(unit1), Ok(unit2)) = (runit1.as_ref(), runit2.as_ref()) {
            let value1 = Value::from_value(unit1.clone(), val);

            match converter.convert(&value1, &unit2) {
                Ok(val) => {
                    *res.write() = val.value;
                    println!("{}", val.value);
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        }
    };

    rsx! {
        table {
            class: "unit-table",
            tr {
                th { "Value" }
                th { "Unit 1" }
                th { "Unit 2" }
                th { "Result" }
                th { "Action" }
                th { "Ok" }
            }
            for (idx, _) in rows.read().iter().enumerate() {
                // key: "{idx}",
                RowUnitTable {
                    on_remove: move |_| remove_row(idx),
                    on_ok: handle_ok,
                }
            }
        }
        button { onclick: add_row, "Add Row" }
    }
}
