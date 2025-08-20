// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    UnitError,
    datatypes::{Dimension, ElementUnit},
};
use futures::executor::block_on;
use turso;

use super::UnitQuery;

pub struct SqlUnitQuery {
    _db: turso::Database,
    conn: turso::Connection,
}

impl SqlUnitQuery {
    const TABLE_NAME: &str = "conversiontable";

    pub async fn new() -> turso::Result<Self> {
        let _db = turso::Builder::new_local(env!("DB_PATH")).build().await?;
        let conn = _db.connect()?;
        let queryself = Self { conn, _db };
        assert!(queryself.check_db_integrity().await);
        Ok(queryself)
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
        dim_name: &str,
    ) -> Result<(String, Dimension), UnitError> {
        //Not implemented yet in Turso, use 2 queries instead
        // let query = format!(
        //     "SELECT dimension_name, mass, length, duration, current, amount, temperature, luminosity
        //     FROM dimension
        //     WHERE dimension_name IN ({});",
        //     Self::get_query_dimension(unit_name)
        // );

        let query = format!(
            "SELECT dimension_name, mass,  duration,length, current, amount, temperature, luminosity
            FROM dimension
            WHERE dimension_name ='{}'",dim_name
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

    async fn check_db_integrity(&self) -> bool {
        let query = "SELECT *  FROM dimension";
        let mut rows = self.conn.query(query, ()).await.unwrap();
        let row = rows.next().await.unwrap();
        if row.is_none() {
            return false;
        }
        let row = row.unwrap();
        row.column_count() == 9
    }
}

impl UnitQuery for SqlUnitQuery {
    fn get_conversion_factor(&self, unit: &ElementUnit) -> Result<f64, UnitError> {
        block_on(self.impl_conversion_factor(&unit.name))
    }

    fn get_unit_list(&self) -> Result<Vec<String>, UnitError> {
        block_on(self.impl_query_unit_list())
    }

    fn get_dimension(&self, unit: &ElementUnit) -> Result<(String, Dimension), UnitError> {
        let dim_name = match &unit.dim {
            Some(name) => name.to_owned(),
            None => block_on(self.impl_get_dim_name(&unit.name))?,
        };

        block_on(self.impl_get_dim_from_unit(&dim_name))
    }

    fn get_dimension_name(&self, p_unit: &ElementUnit) -> Result<String, UnitError> {
        block_on(self.impl_get_dim_name(&p_unit.name))
    }
}
