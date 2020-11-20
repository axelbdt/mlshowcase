pub mod datasets;

#[database("mlshowcase")]
pub struct DbConn(diesel::PgConnection);
