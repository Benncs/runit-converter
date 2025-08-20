// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    UnitError,
    datatypes::{Dimension, ElementUnit, Unit},
};

mod sql;
pub use sql::SqlUnitQuery;
pub trait UnitQuery {
    fn get_conversion_factor(&self, unit: &ElementUnit) -> Result<f64, UnitError>;

    fn get_unit_list(&self) -> Result<Vec<String>, UnitError>;

    fn get_dimension(&self, unit: &ElementUnit) -> Result<(String, Dimension), UnitError>;

    fn get_dimension_name(&self, p_unit: &ElementUnit) -> Result<String, UnitError>;
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_dimension_name() {
        let c = SqlUnitQuery::new().await.unwrap();
        let pu = ElementUnit::new("kg", 1.);
        let name = c.get_dimension_name(&pu).unwrap();
        assert!(name == *"mass");
        let pu = ElementUnit::new("s", 1.);
        let name = c.get_dimension_name(&pu).unwrap();
        assert!(name == *"duration");

        let pu = ElementUnit::new("FALSEUNIT", 99.);
        assert!(c.get_dimension_name(&pu).is_err());
    }

    #[tokio::test]
    async fn test_conversion_factor() {
        let c = SqlUnitQuery::new().await.unwrap();
        let pu = ElementUnit::new("kg", 1.);
        let cv = c.get_conversion_factor(&pu).unwrap();
        assert!(cv == 1.);
        let pu = ElementUnit::new("g", 1.);
        let cv = c.get_conversion_factor(&pu).unwrap();
        assert!(cv == 1e-3);

        let pu = ElementUnit::new("FALSEUNIT", 99.);
        assert!(c.get_conversion_factor(&pu).is_err());
    }
}
