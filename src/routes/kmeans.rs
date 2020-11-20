use crate::data::generation;
use crate::data::processing;
use crate::db::datasets;
use crate::db::DbConn;

use super::APIResult;

use rocket_contrib::json::Json;

#[get("/kmeans?<dataset_id>&<k>")]
pub fn kmeans(conn: DbConn, dataset_id: i32, k: usize) -> Option<Json<APIResult>> {
    let dataset = match datasets::find_by_id(&conn, dataset_id) {
        Some(dataset) => dataset,
        None => return None,
    };
    let data = match generation::generate_dataset(&dataset) {
        Ok(data) => data,
        Err(_) => return None,
    };
    let centroids = processing::kmeans(k, &data);
    Some(Json(centroids.into()))
}
