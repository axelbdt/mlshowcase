use diesel::sql_types::Json;
use diesel::Queryable;

#[derive(Queryable)]
pub struct Dataset {
    pub id: i32,
    pub seed: i64,
    pub gmm: Json,
}
