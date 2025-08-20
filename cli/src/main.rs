// SPDX-License-Identifier: GPL-3.0-or-later

use std::process::ExitCode;

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
    pub unit2: Option<String>,
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
async fn main() -> ExitCode {
    let args = GenArgs::parse();

    match args.mode {
        Mode::Convert(ConvertArgs {
            value,
            unit1,

            unit2,
        }) => {
            let (parser, factory, converter) = construct_all().await;

            let runit1 = factory.parse_fill(&parser, &unit1);
            let runit2 = factory.parse_fill(&parser, &unit2);

            if let (Ok(unit1), Ok(unit2)) = (runit1.as_ref(), runit2.as_ref()) {
                let value1 = Value::from_value(unit1.clone(), value);

                match converter.convert(&value1, unit2) {
                    Ok(val) => {
                        println!("{}", val.value);
                        return ExitCode::SUCCESS;
                    }
                    Err(e) => {
                        if args.verbose {
                            println!("{}", e);
                            return ExitCode::FAILURE;
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
            return ExitCode::FAILURE;
        }
        Mode::Dim(DimArgs { unit1, unit2 }) => {
            let (parser, factory, converter) = construct_all().await;
            let runit1 = factory.parse_fill(&parser, &unit1);

            if let Err(e) = runit1 {
                println!("Unit1 : {}", e);
                return ExitCode::FAILURE;
            }

            if let Some(unit2) = unit2 {
                let runit2 = factory.parse_fill(&parser, &unit2);
                if let Err(e) = runit2 {
                    println!("Unit2 : {}", e);
                    return ExitCode::FAILURE;
                }
                let flag = if converter
                    .are_same_dimension(&runit1.unwrap(), &runit2.unwrap())
                    .0
                {
                    0
                } else {
                    1
                };

                println!("{}", flag);
                //If true/false same of success/failure but in this case this is not interpreted as succes neitheir failure it's boolean flags

                return flag.into();
            } else {
                //Safe to unwrap before tested before
                println!("{}", converter.get_dimension(&runit1.unwrap()));
                return ExitCode::SUCCESS;
            }
        }
        Mode::List => {
            let c = std::rc::Rc::new(SqlUnitQuery::new().await.unwrap());
            let names = c.get_unit_list().unwrap();
            names.iter().for_each(|unit| {
                println!("{}", unit);
            });
        }
    }
    return ExitCode::SUCCESS;
}
