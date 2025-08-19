mod datatypes;
pub mod unitquery;

pub use datatypes::{ElementUnit, Unit};
use unitquery::UnitQuery;

pub trait UnitConverter {
    fn is_valid_unit(&mut self, unit: &Unit) -> bool;
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
}
