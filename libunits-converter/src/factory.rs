// SPDX-License-Identifier: GPL-3.0-or-later

use std::rc::Rc;

use crate::{ElementUnit, Unit, UnitError, UnitParser, unitquery::UnitQuery};

pub trait UnitFactory {
    fn construct_unit(&self, name: &str, exp: f64) -> Result<ElementUnit, UnitError>;
    fn fill(&self, unit: &mut ElementUnit) -> Result<(), UnitError>;
    fn parse_fill<T: UnitParser>(&self, parser: &T, text: &str) -> Result<Unit, UnitError>;
}

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

        for pu in unit.partials.iter_mut() {
            self.fill(pu)?;
        }

        Ok(unit)
    }
}

#[cfg(test)]
mod test {
    use crate::unitquery::SqlUnitQuery;

    use super::*;
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
