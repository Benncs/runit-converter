mod datatypes;
pub mod unitquery;

use datatypes::Dimension;
pub use datatypes::{ElementUnit, Unit, Value};
use unitquery::UnitQuery;
pub enum UnitMatch {
    Different,
    Same,
    Equal,
}

pub trait UnitConverter {
    fn is_valid_unit(&mut self, unit: &Unit) -> bool;
    fn are_same_dimension(&self, unit1: &Unit, unit2: &Unit) -> bool;
    fn get_dimension(&self, unit: &Unit) -> Dimension;
    fn convert(&self, unit: &Value, unit: &Unit) -> Result<Value, ()>;
    fn get_conversion_factor(&self, unit: &Unit) -> Result<f64, ()>;
}

pub struct MainConverter<T: UnitQuery> {
    query: T,
    ulist: Option<Vec<String>>,
}

impl<T: UnitQuery> MainConverter<T> {
    pub fn new(query: T) -> Self {
        Self { query, ulist: None }
    }
}

impl<T: UnitQuery> UnitConverter for MainConverter<T> {
    fn is_valid_unit(&mut self, unit: &Unit) -> bool {
        if self.ulist.is_none() {
            self.ulist = Some(self.query.get_unit_list().unwrap()); //Do not use ok() to keep result check with unwrap
        }
        unit.partials
            .iter()
            .all(|p_u| self.ulist.as_ref().unwrap().contains(&p_u.name))
    }

    fn get_conversion_factor(&self, unit: &Unit) -> Result<f64, ()> {
        let mut cf = 1.;
        for partial in &unit.partials {
            let c = self.query.get_conversion_factor(partial).unwrap();

            cf *= c.powf(partial.exp());
        }
        Ok(cf)
    }

    fn get_dimension(&self, unit: &Unit) -> Dimension {
        let mut dimension = Dimension::default();
        for partial in &unit.partials {
            dimension = dimension.dot(&self.query.get_dimension(partial).unwrap(), partial.exp());
        }
        dimension
    }

    fn are_same_dimension(&self, unit1: &Unit, unit2: &Unit) -> bool {
        let d1 = self.get_dimension(unit1);
        let d2 = self.get_dimension(unit2);
        d1 == d2
    }

    fn convert(&self, val: &Value, unit: &Unit) -> Result<Value, ()> {
        if self.are_same_dimension(&val.unit, unit) {
            let cf1 = self.get_conversion_factor(&val.unit).unwrap();
            let cf2 = self.get_conversion_factor(unit).unwrap();
            Ok(Value::from_value(unit.clone(), val.value * cf1 / cf2))
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::unitquery::SqlUnitQuery;
    use super::*;

    #[tokio::test]
    async fn test_valid_unit() {
        let c = SqlUnitQuery::new().await.unwrap();
        let mut converter = MainConverter::new(c);

        let pu = ElementUnit::new("kg", 1.);
        assert!(converter.is_valid_unit(&pu.into()));

        let pu = ElementUnit::new("kg1", 1.);
        assert!(!converter.is_valid_unit(&pu.into()));
    }

    #[tokio::test]
    async fn test_same_dimension() {
        let c = SqlUnitQuery::new().await.unwrap();
        let mut converter = MainConverter::new(c);
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
        let c = SqlUnitQuery::new().await.unwrap();
        let converter = MainConverter::new(c);
        let pu = ElementUnit::new("kg", 1.);

        assert!(converter.get_dimension(&pu.into()) == Dimension([1, 0, 0, 0, 0, 0, 0]));

        let full_unit =
            Unit::from_vec(vec![ElementUnit::new("kg", 1.), ElementUnit::new("s", -1.)]);
        assert!(converter.get_dimension(&full_unit.into()) == Dimension([1, -1, 0, 0, 0, 0, 0]));
    }

    #[tokio::test]
    async fn test_get_coefficient_factor() {
        let c = SqlUnitQuery::new().await.unwrap();
        let converter = MainConverter::new(c);
        let pu = ElementUnit::new("g", 1.);
        let full_unit = Unit::from_vec(vec![ElementUnit::new("g", 1.), ElementUnit::new("h", -1.)]);

        assert!(converter.get_conversion_factor(&pu.into()).unwrap() == 1e-3);
        assert!(converter.get_conversion_factor(&full_unit).unwrap() == 1e-3 / 3600.);
    }

    #[tokio::test]
    async fn test_convert() {
        let c = SqlUnitQuery::new().await.unwrap();
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
