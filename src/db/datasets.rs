use crate::models::dataset::Dataset;
use crate::schema::datasets::dsl::*;

use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn find_by_id(conn: &PgConnection, dataset_id: i32) -> Option<Dataset> {
    let result = datasets.filter(id.eq(dataset_id)).first(conn).ok()?;
    Some(result)
}
