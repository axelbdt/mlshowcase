use diesel::deserialize::{FromSql, Result};
use diesel::pg::Pg;
use diesel::sql_types::Jsonb;
use diesel::Queryable;

use na::{Matrix2, Vector2};

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Queryable)]
pub struct Dataset {
    pub id: i32,
    pub seed: i64,
    pub gmm: Vec<Gaussian>,
}

#[derive(Deserialize, Serialize)]
pub struct Gaussian {
    pub mean: Vector2<f32>,
    pub cov: Matrix2<f32>,
}

impl FromSql<Jsonb, Pg> for Gaussian {
    fn from_sql(bytes: Option<&[u8]>) -> Result<Self> {
        let value = <serde_json::Value as FromSql<Jsonb, Pg>>::from_sql(bytes)?;
        Ok(serde_json::from_value(value)?)
    }
}
