// SPDX-License-Identifier: GPL-3.0-or-later

mod datatypes;
mod error;
mod parser;
pub mod unitquery;

pub enum UnitMatch {
    Different,
    Same,
    Equal,
}
mod factory;

use datatypes::Dimension;
pub use factory::{MainUnitFactory, UnitFactory};

pub use datatypes::{ElementUnit, Unit, Value};
pub use error::UnitError;
pub use parser::{InlineUnitParser, UnitParser};
use std::rc::Rc;
use unitquery::{SqlUnitQuery, UnitQuery};

pub trait UnitConverter {
    fn is_valid_unit(&mut self, unit: &Unit) -> bool;
    fn are_same_dimension(&self, unit1: &Unit, unit2: &Unit) -> (bool, Dimension, Dimension);
    fn get_dimension(&self, unit: &Unit) -> Dimension;
    fn get_dimension_mut(&self, unit: &mut Unit) -> Dimension;
    fn convert(&self, unit1: &Value, unit2: &Unit) -> Result<Value, UnitError>;
    fn convert_mut(&self, unit1: &mut Value, unit2: &mut Unit) -> Result<Value, UnitError>;
    fn get_conversion_factor(&self, unit: &Unit) -> Result<f64, UnitError>;
}

pub struct MainConverter<T: UnitQuery> {
    query: Rc<T>,
    ulist: Option<Vec<String>>,
}

impl<T: UnitQuery> MainConverter<T> {
    pub fn new(query: Rc<T>) -> Self {
        Self { query, ulist: None }
    }
    fn fold_dimension<'a, I, P, F>(&self, partials: I, mut on_partial: F) -> Dimension
    where
        I: IntoIterator<Item = P>,
        P: 'a + std::borrow::Borrow<ElementUnit>,
        F: FnMut(P, &str),
    {
        let mut dimension = Dimension::default();

        for partial in partials {
            let partial_ref: &ElementUnit = partial.borrow();
            let (name, dim) = self.query.get_dimension(partial_ref).unwrap();
            dimension = dimension.dot(&dim, partial_ref.exp());
            on_partial(partial, &name);
        }

        dimension
    }
}

//

impl<T: UnitQuery> UnitConverter for MainConverter<T> {
    fn is_valid_unit(&mut self, unit: &Unit) -> bool {
        if self.ulist.is_none() {
            self.ulist = Some(self.query.get_unit_list().unwrap()); //Do not use ok() to keep result check with unwrap
        }
        unit.partials
            .iter()
            .all(|p_u| self.ulist.as_ref().unwrap().contains(&p_u.name))
    }

    fn get_conversion_factor(&self, unit: &Unit) -> Result<f64, UnitError> {
        let mut cf = 1.;
        for partial in &unit.partials {
            let c = self.query.get_conversion_factor(partial).unwrap();

            cf *= c.powf(partial.exp());
        }
        Ok(cf)
    }

    fn get_dimension(&self, unit: &Unit) -> Dimension {
        self.fold_dimension(unit.partials.iter(), |_, _| {})
    }

    fn get_dimension_mut(&self, unit: &mut Unit) -> Dimension {
        self.fold_dimension(unit.partials.iter_mut(), |p, name| {
            p.dim = Some(name.to_string());
        })
    }

    fn are_same_dimension(&self, unit1: &Unit, unit2: &Unit) -> (bool, Dimension, Dimension) {
        let d1 = self.get_dimension(unit1);
        let d2 = self.get_dimension(unit2);
        (d1 == d2, d1, d2)
    }

    fn convert(&self, val: &Value, unit: &Unit) -> Result<Value, UnitError> {
        if self.are_same_dimension(&val.unit, unit).0 {
            let cf1 = self.get_conversion_factor(&val.unit).unwrap();
            let cf2 = self.get_conversion_factor(unit).unwrap();
            Ok(Value::from_value(unit.clone(), val.value * cf1 / cf2))
        } else {
            Err(UnitError::BadDimension)
        }
    }

    fn convert_mut(&self, unit1: &mut Value, unit2: &mut Unit) -> Result<Value, UnitError> {
        todo!()
    }
}

pub async fn construct_all() -> (
    InlineUnitParser,
    MainUnitFactory<SqlUnitQuery>,
    MainConverter<SqlUnitQuery>,
) {
    let c = std::rc::Rc::new(SqlUnitQuery::new().await.unwrap());
    let parser = InlineUnitParser::default();
    let factory = MainUnitFactory::new(c.clone());
    let converter = MainConverter::new(c);
    (parser, factory, converter)
}

#[cfg(test)]
mod test {
    use super::unitquery::SqlUnitQuery;
    use super::*;

    #[tokio::test]
    async fn test_valid_unit() {
        let c = Rc::new(SqlUnitQuery::new().await.unwrap());
        let mut converter = MainConverter::new(c);

        let pu = ElementUnit::new("kg", 1.);
        assert!(converter.is_valid_unit(&pu.into()));

        let pu = ElementUnit::new("kg1", 1.);
        assert!(!converter.is_valid_unit(&pu.into()));
    }

    #[tokio::test]
    async fn test_same_dimension() {
        let c = Rc::new(SqlUnitQuery::new().await.unwrap());
        let converter = MainConverter::new(c);
        let pu = ElementUnit::new("kg", 1.);
        let pu2 = ElementUnit::new("g", 1.);

        assert!(converter.are_same_dimension(&pu.into(), &pu2.into()).0);

        let full_unit =
            Unit::from_vec(vec![ElementUnit::new("kg", 1.), ElementUnit::new("s", -1.)]);
        let full_unit2 =
            Unit::from_vec(vec![ElementUnit::new("g", 1.), ElementUnit::new("h", -1.)]);

        assert!(converter.are_same_dimension(&full_unit, &full_unit2).0);
    }

    #[tokio::test]
    async fn test_get_dimension() {
        let c = Rc::new(SqlUnitQuery::new().await.unwrap());
        let converter = MainConverter::new(c);
        let pu = ElementUnit::new("kg", 1.);

        assert!(converter.get_dimension(&pu.into()) == Dimension([1, 0, 0, 0, 0, 0, 0]));

        let full_unit =
            Unit::from_vec(vec![ElementUnit::new("kg", 1.), ElementUnit::new("s", -1.)]);
        assert!(converter.get_dimension(&full_unit.into()) == Dimension([1, -1, 0, 0, 0, 0, 0]));
    }

    #[tokio::test]
    async fn test_get_coefficient_factor() {
        let c = Rc::new(SqlUnitQuery::new().await.unwrap());
        let converter = MainConverter::new(c);
        let pu = ElementUnit::new("g", 1.);
        let full_unit = Unit::from_vec(vec![ElementUnit::new("g", 1.), ElementUnit::new("h", -1.)]);

        assert!(converter.get_conversion_factor(&pu.into()).unwrap() == 1e-3);
        assert!(converter.get_conversion_factor(&full_unit).unwrap() == 1e-3 / 3600.);
    }

    #[tokio::test]
    async fn test_convert() {
        let c = Rc::new(SqlUnitQuery::new().await.unwrap());
        let converter = MainConverter::new(c);
        let pu = ElementUnit::new("g", 1.);
        let pu2 = ElementUnit::new("kg", 1.);
        let pu_wrong = ElementUnit::new("m", 1.);

        let value = Value::from_value(pu.into(), 5.0);

        assert!(converter.convert(&value, &pu2.into()).unwrap().value == 5e-3);

        assert!(converter.convert(&value, &pu_wrong.into()).is_err());

        let full_unit = Unit::from_vec(vec![ElementUnit::new("g", 2.), ElementUnit::new("h", -1.)]);
        let full_unit2 =
            Unit::from_vec(vec![ElementUnit::new("kg", 2.), ElementUnit::new("h", -1.)]);

        let value = Value::from_value(full_unit, 5.0);

        assert!(converter.convert(&value, &full_unit2).unwrap().value == 5. * 1e-6);
    }
}
