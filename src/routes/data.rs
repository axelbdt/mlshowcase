use crate::db::datasets;
use crate::db::DbConn;
use crate::handlers::generation;
use crate::models::data::DataJson;

use rocket_contrib::json::Json;

#[get("/data?<dataset_id>")]
pub fn data(conn: DbConn, dataset_id: i32) -> Option<Json<DataJson>> {
    let dataset = match datasets::find_by_id(&conn, dataset_id) {
        Some(dataset) => dataset,
        None => return None,
    };
    let data = match generation::generate_dataset(&dataset) {
        Ok(data) => data,
        Err(_) => return None,
    };
    Some(Json(data.into()))
}
