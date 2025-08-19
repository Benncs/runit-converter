use runits_converter::unitquery::{SqlUnitQuery, UnitQuery};

use runits_converter::*;

#[tokio::main]

async fn main() {
    let c = SqlUnitQuery::new().await.unwrap();
    let names = c.get_unit_list().unwrap();

    println!("{:?}", names);

    let pu = ElementUnit::new("kg1");

    let mut converter = MainConverter::new(c);

    println!("{:?}", converter.is_valid_unit(&pu.into()));
    // let name = c.get_dimension_name(&pu);

    // println!("{:?}", name);
}
