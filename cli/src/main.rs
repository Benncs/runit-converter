// SPDX-License-Identifier: GPL-3.0-or-later

use std::process::exit;

use clap::{Parser, Subcommand};
use libunits_converter::unitquery::{SqlUnitQuery, UnitQuery};
use libunits_converter::*;

#[derive(Parser, Default, Clone)]
pub struct ConvertArgs {
    pub value: f64,

    pub unit1: String,
    pub unit2: String,
}

#[derive(Parser, Default, Clone)]
pub struct DimArgs {
    pub unit1: String,
    pub unit2: String,
}

#[derive(Subcommand, Clone)]
pub enum Mode {
    Convert(ConvertArgs),
    Dim(DimArgs),
    List,
}
#[derive(Parser, Clone)]
#[command(
    name = "RUnit-Converter",
    author,
    version,
    about = env!("CARGO_PKG_DESCRIPTION"),
    help_template = "\
{name} {version}

{about}

USAGE:
    {usage}

OPTIONS:
{options}

COMMANDS:
{subcommands}

By {author}
"
)]
pub struct GenArgs {
    #[clap(long, short, action=clap::ArgAction::SetTrue)]
    verbose: bool,
    #[clap(subcommand)]
    pub mode: Mode,
}

#[tokio::main]
async fn main() {
    let args = GenArgs::parse();

    match args.mode {
        Mode::Convert(ConvertArgs {
            value,
            unit1,

            unit2,
        }) => {
            let c = std::rc::Rc::new(SqlUnitQuery::new().await.unwrap());
            let parser = InlineUnitParser::default();
            let factory = MainUnitFactory::new(c.clone());
            let converter = MainConverter::new(c.clone());

            let runit1 = factory.parse_fill(&parser, &unit1);
            let runit2 = factory.parse_fill(&parser, &unit2);

            if let (Ok(unit1), Ok(unit2)) = (runit1.as_ref(), runit2.as_ref()) {
                let value1 = Value::from_value(unit1.clone(), value);

                match converter.convert(&value1, unit2) {
                    Ok(val) => {
                        println!("{}", val.value);
                        exit(0);
                    }
                    Err(e) => {
                        if args.verbose {
                            println!("{}", e);
                            exit(-1);
                        }
                    }
                }
            } else if args.verbose {
                if let Err(e) = runit1 {
                    println!("Unit1 : {}", e)
                }
                if let Err(e) = runit2 {
                    println!("Unit2 : {}", e)
                }
            }
            exit(-1)
        }
        Mode::Dim(DimArgs { unit1, unit2 }) => {}
        Mode::List => {
            let c = std::rc::Rc::new(SqlUnitQuery::new().await.unwrap());
            let names = c.get_unit_list().unwrap();
            names.iter().for_each(|unit| {
                println!("{}", unit);
            });
        }
    }
}
