use crate::db::datasets;
use crate::db::DbConn;
use crate::handlers::generation;
use crate::handlers::processing;
use crate::models::data::DataJson;

use rocket_contrib::json::Json;

#[get("/kmeans?<dataset_id>&<k>")]
pub fn kmeans(conn: DbConn, dataset_id: i32, k: usize) -> Option<Json<DataJson>> {
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
