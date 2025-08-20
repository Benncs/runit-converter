use runits_converter::unitquery::{SqlUnitQuery, UnitQuery};

use runits_converter::*;

#[tokio::main]
async fn main() {
    let c = std::rc::Rc::new(SqlUnitQuery::new().await.unwrap());
    let names = c.get_unit_list().unwrap();

    println!("{:?}", names);

    let pu = ElementUnit::new("kg", 1.);
    let pu2 = ElementUnit::new("g", 1.);

    let mut converter = MainConverter::new(c.clone());

    let r = converter.are_same_dimension(&pu.into(), &pu2.into());
    // let name = c.get_dimension_name(&pu);

    println!("{:?}", r);
}
