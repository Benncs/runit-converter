mod datatypes;
mod error;
pub mod unitquery;
use datatypes::Dimension;
pub use datatypes::{ElementUnit, Unit, Value};
pub use error::UnitError;
use std::rc::Rc;
use unitquery::UnitQuery;
mod parser;
pub use parser::{InlineUnitParser, UnitParser};
pub enum UnitMatch {
    Different,
    Same,
    Equal,
}

pub trait UnitFactory {
    fn construct_unit(&self, name: &str, exp: f64) -> Result<ElementUnit, UnitError>;
    fn fill(&self, unit: &mut ElementUnit) -> Result<(), UnitError>;
    fn parse_fill<T: UnitParser>(&self, parser: &T, text: &str) -> Result<Unit, UnitError>;
}

pub trait UnitConverter {
    fn is_valid_unit(&mut self, unit: &Unit) -> bool;
    fn are_same_dimension(&self, unit1: &Unit, unit2: &Unit) -> bool;
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

    fn are_same_dimension(&self, unit1: &Unit, unit2: &Unit) -> bool {
        let d1 = self.get_dimension(unit1);
        let d2 = self.get_dimension(unit2);
        d1 == d2
    }

    fn convert(&self, val: &Value, unit: &Unit) -> Result<Value, UnitError> {
        if self.are_same_dimension(&val.unit, unit) {
            let cf1 = self.get_conversion_factor(&val.unit).unwrap();
            let cf2 = self.get_conversion_factor(unit).unwrap();
            Ok(Value::from_value(unit.clone(), val.value * cf1 / cf2))
        } else {
            Err(UnitError::BadUnit("".to_owned()))
        }
    }

    fn convert_mut(&self, unit1: &mut Value, unit2: &mut Unit) -> Result<Value, UnitError> {
        todo!()
    }
}

// impl<T: UnitQuery> UnitFactory for MainConverter<T> {
// //     fn construct_unit(&self, name: &str, exp: f64) -> Result<ElementUnit, UnitError> {
// //         let mut unit = ElementUnit::new(name, exp);
// //         unit.set_dim(&self.query.get_dimension_name(&unit)?);
// //         unit.set_factor(self.query.get_conversion_factor(&unit)?);
// //         Ok(unit)
// //     }
// // }
//
pub struct MainUnitFactory<T: UnitQuery> {
    query: Rc<T>,
}

impl<T: UnitQuery> MainUnitFactory<T> {
    pub fn new(query: Rc<T>) -> Self {
        Self { query }
    }
}

impl<T: UnitQuery> UnitFactory for MainUnitFactory<T> {
    fn construct_unit(&self, name: &str, exp: f64) -> Result<ElementUnit, UnitError> {
        let mut unit = ElementUnit::new(name, exp);
        self.fill(&mut unit)?;
        Ok(unit)
    }

    fn fill(&self, unit: &mut ElementUnit) -> Result<(), UnitError> {
        unit.set_dim(&self.query.get_dimension_name(unit)?);
        unit.set_factor(self.query.get_conversion_factor(unit)?);
        Ok(())
    }
    fn parse_fill<G: UnitParser>(&self, parser: &G, text: &str) -> Result<Unit, UnitError> {
        let mut unit = parser.parse_unit(text)?;
        unit.partials.iter_mut().for_each(|mut pu| {
            self.fill(&mut pu);
        });
        Ok(unit)
    }
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

        assert!(converter.are_same_dimension(&pu.into(), &pu2.into()));

        let full_unit =
            Unit::from_vec(vec![ElementUnit::new("kg", 1.), ElementUnit::new("s", -1.)]);
        let full_unit2 =
            Unit::from_vec(vec![ElementUnit::new("g", 1.), ElementUnit::new("h", -1.)]);

        assert!(converter.are_same_dimension(&full_unit, &full_unit2));
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

    #[tokio::test]
    async fn test_construct() {
        let c = Rc::new(SqlUnitQuery::new().await.unwrap());
        let converter = MainUnitFactory::new(c);
        let pu = converter.construct_unit("g", 1.).unwrap();
        let pu2 = converter.construct_unit("kg", 1.).unwrap();
        let pu3 = converter.construct_unit("m", 1.).unwrap();

        assert!(pu.dim == Some("mass".to_owned()));
        assert!(pu2.dim == Some("mass".to_owned()));
        assert!(pu3.dim == Some("length".to_owned()));
        assert!(pu.get_factor() == 1e-3);
    }
}
