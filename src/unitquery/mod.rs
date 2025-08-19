use crate::datatypes::{Dimension, ElementUnit, Unit};
use futures::executor::block_on;
use turso;

pub trait UnitQuery {
    fn get_conversion_factor(&self, unit: &Unit) -> Option<f64>;

    fn get_unit_list(&self) -> Result<Vec<String>, ()>;

    fn get_dimension(&self, unit: &ElementUnit) -> Result<Dimension, ()>;

    fn get_dimension_name(&self, p_unit: &ElementUnit) -> Result<String, ()>;

    fn are_same_dimension(&self, unit1: &Unit, unit2: Unit) -> bool;
}

pub struct SqlUnitQuery {
    _db: turso::Database,
    conn: turso::Connection,
}

impl SqlUnitQuery {
    const TABLE_NAME: &str = "conversiontable";

    pub async fn new() -> turso::Result<Self> {
        let _db = turso::Builder::new_local("data/sqlite.db").build().await?;
        let conn = _db.connect()?;
        Ok(Self { conn, _db })
    }

    async fn impl_query_unit_list(&self) -> Result<Vec<String>, ()> {
        let query = format!(
            "SELECT unit_name, dimension_name FROM {};",
            Self::TABLE_NAME
        );
        let mut rows = self.conn.query(&query, ()).await.unwrap();
        let mut names = Vec::new();
        while let Some(row) = rows.next().await.unwrap() {
            let name = row.get_value(0).unwrap();
            names.push(name.as_text().unwrap().to_owned());
        }

        if names.is_empty() { Err(()) } else { Ok(names) }
    }

    fn get_query_dimension(unit_name: &str) -> String {
        format!(
            "SELECT dimension_name FROM conversiontable WHERE unit_name = '{}'",
            unit_name
        )
    }

    async fn impl_get_dim_name(&self, unit_name: &str) -> Result<String, ()> {
        let query = Self::get_query_dimension(unit_name);
        let mut rows = self.conn.query(&query, ()).await.unwrap();
        let row = rows.next().await.unwrap();
        if row.is_none() {
            return Err(());
        }
        let name = row
            .unwrap()
            .get_value(0)
            .unwrap()
            .as_text()
            .unwrap()
            .to_owned();

        if rows.next().await.unwrap().is_some() {
            return Err(());
        }

        Ok(name)
    }

    async fn impl_get_dim_from_unit(&self, unit_name: &str) -> Result<(String, Dimension), ()> {
        //Not implemented yet in Turso, use 2 queries instead
        // let query = format!(
        //     "SELECT dimension_name, mass, length, duration, current, amount, temperature, luminosity
        //     FROM dimension
        //     WHERE dimension_name IN ({});",
        //     Self::get_query_dimension(unit_name)
        // );

        let name = self.impl_get_dim_name(unit_name).await?;
        let query = format!(
            "SELECT dimension_name, mass,  duration,length, current, amount, temperature, luminosity
            FROM dimension
            WHERE dimension_name ='{}'",name
            // Self::get_query_dimension(unit_name)
        );
        let mut rows = self.conn.query(&query, ()).await.unwrap();
        let row = rows.next().await.unwrap().unwrap();
        assert!(row.column_count() == 8);
        if rows.next().await.unwrap().is_some() {
            return Err(());
        }

        let dimension_name = row.get_value(0).unwrap().as_text().unwrap().to_owned();
        let mut dimension = Dimension::default();
        for i in 1..dimension.0.len() {
            dimension.0[i - 1] = row.get_value(i).unwrap().as_real().unwrap().to_owned() as i32;
        }

        Ok((dimension_name, dimension))
    }

    async fn check_db_integrity() -> turso::Result<bool> {
        todo!();
    }
}

impl UnitQuery for SqlUnitQuery {
    fn get_conversion_factor(&self, unit: &Unit) -> Option<f64> {
        todo!()
    }

    fn get_unit_list(&self) -> Result<Vec<String>, ()> {
        block_on(self.impl_query_unit_list())
    }

    fn get_dimension(&self, unit: &ElementUnit) -> Result<Dimension, ()> {
        if unit.dim.is_none() {
            let (_name, dim) = block_on(self.impl_get_dim_from_unit(&unit.name)).unwrap();
            Ok(dim)
        } else {
            todo!()
        }
    }

    fn get_dimension_name(&self, p_unit: &ElementUnit) -> Result<String, ()> {
        block_on(self.impl_get_dim_name(&p_unit.name))
    }

    fn are_same_dimension(&self, unit1: &Unit, unit2: Unit) -> bool {
        todo!()
    }
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
}
