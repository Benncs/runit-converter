use crate::{
    UnitError,
    datatypes::{Dimension, ElementUnit, Unit},
};
use futures::executor::block_on;
use turso;

pub trait UnitQuery {
    fn get_conversion_factor(&self, unit: &ElementUnit) -> Result<f64, UnitError>;

    fn get_unit_list(&self) -> Result<Vec<String>, UnitError>;

    fn get_dimension(&self, unit: &ElementUnit) -> Result<Dimension, UnitError>;

    fn get_dimension_name(&self, p_unit: &ElementUnit) -> Result<String, UnitError>;
}

pub struct SqlUnitQuery {
    _db: turso::Database,
    conn: turso::Connection,
}

impl SqlUnitQuery {
    const TABLE_NAME: &str = "conversiontable";

    pub async fn new() -> turso::Result<Self> {
        let _db = turso::Builder::new_local(env!("DB_PATH")).build().await?;
        let conn = _db.connect()?;
        Ok(Self { conn, _db })
    }

    async fn impl_query_unit_list(&self) -> Result<Vec<String>, UnitError> {
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

        if names.is_empty() {
            Err(UnitError::Custom("Empty databse".to_owned()))
        } else {
            Ok(names)
        }
    }

    fn get_query_dimension(unit_name: &str) -> String {
        format!(
            "SELECT dimension_name FROM conversiontable WHERE unit_name = '{}'",
            unit_name
        )
    }

    async fn impl_get_dim_name(&self, unit_name: &str) -> Result<String, UnitError> {
        let query = Self::get_query_dimension(unit_name);
        let mut rows = self.conn.query(&query, ()).await.unwrap();
        let row = rows.next().await.unwrap();
        if row.is_none() {
            return Err(UnitError::Query(
                "SqlQuery: should have exactly one result".to_owned(),
            ));
        }
        let name = row
            .unwrap()
            .get_value(0)
            .unwrap()
            .as_text()
            .unwrap()
            .to_owned();

        if rows.next().await.unwrap().is_some() {
            return Err(UnitError::Query(
                "SqlQuery: should have exactly one result".to_owned(),
            ));
        }

        Ok(name)
    }

    async fn impl_get_dim_from_unit(
        &self,
        unit_name: &str,
    ) -> Result<(String, Dimension), UnitError> {
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
            return Err(UnitError::Query(
                "SqlQuery: should have exactly one result".to_owned(),
            ));
        }

        let dimension_name = row.get_value(0).unwrap().as_text().unwrap().to_owned();
        let mut dimension = Dimension::default();
        for i in 1..dimension.0.len() {
            dimension.0[i - 1] = row.get_value(i).unwrap().as_real().unwrap().to_owned() as i32;
        }

        Ok((dimension_name, dimension))
    }

    async fn impl_conversion_factor(&self, unit_name: &str) -> Result<f64, UnitError> {
        let query = format!(
            "SELECT conversionfactor  FROM conversiontable WHERE unit_name = '{}'",
            unit_name
        );

        let mut rows = self.conn.query(&query, ()).await.unwrap();
        let row = rows.next().await.unwrap();
        if row.is_none() {
            return Err(UnitError::Query(
                "SqlQuery: should have exactly one result".to_owned(),
            ));
        }
        let row = row.unwrap();
        assert!(row.column_count() == 1);
        if rows.next().await.unwrap().is_some() {
            return Err(UnitError::Query(
                "SqlQuery: should have exactly one result".to_owned(),
            ));
        }

        Ok(row.get_value(0).unwrap().as_real().unwrap().to_owned())
    }

    async fn check_db_integrity() -> turso::Result<bool> {
        todo!();
    }
}

impl UnitQuery for SqlUnitQuery {
    fn get_conversion_factor(&self, unit: &ElementUnit) -> Result<f64, UnitError> {
        block_on(self.impl_conversion_factor(&unit.name))
    }

    fn get_unit_list(&self) -> Result<Vec<String>, UnitError> {
        block_on(self.impl_query_unit_list())
    }

    fn get_dimension(&self, unit: &ElementUnit) -> Result<Dimension, UnitError> {
        if unit.dim.is_none() {
            let (_name, dim) = block_on(self.impl_get_dim_from_unit(&unit.name)).unwrap();
            Ok(dim)
        } else {
            todo!()
        }
    }

    fn get_dimension_name(&self, p_unit: &ElementUnit) -> Result<String, UnitError> {
        block_on(self.impl_get_dim_name(&p_unit.name))
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
