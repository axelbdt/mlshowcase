use crate::models::dataset::Dataset;
use crate::schema::datasets;

use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn find_by_id(conn: &PgConnection, id: i32) -> Option<Dataset> {
    let result = datasets::table
        .filter(datasets::id.eq(id))
        .first(conn)
        .ok()?;
    Some(result)
}
