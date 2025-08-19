mod datatypes;
pub mod unitquery;

use datatypes::Dimension;
pub use datatypes::{ElementUnit, Unit};
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
        let mut converter = MainConverter::new(c);
        let pu = ElementUnit::new("kg", 1.);

        assert!(converter.get_dimension(&pu.into()) == Dimension([1, 0, 0, 0, 0, 0, 0]));

        let full_unit =
            Unit::from_vec(vec![ElementUnit::new("kg", 1.), ElementUnit::new("s", -1.)]);
        assert!(converter.get_dimension(&full_unit.into()) == Dimension([1, -1, 0, 0, 0, 0, 0]));
    }
}
