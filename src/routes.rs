use crate::db::datasets;
use crate::error::Error;
use crate::generation;
use crate::generation::{Data, Sample};
use crate::processing;
use crate::DbConn;

use rocket_contrib::json::Json;

use serde::Serialize;

#[derive(Serialize)]
pub enum APIResult {
    Data(Vec<Point>),
    Error(Error),
}

impl From<Error> for APIResult {
    fn from(e: Error) -> Self {
        APIResult::Error(e)
    }
}

impl From<Data> for APIResult {
    fn from(data: Data) -> Self {
        let points = data.into_iter().map(|s| s.into()).collect();
        APIResult::Data(points)
    }
}

#[derive(Serialize)]
pub struct Point {
    x: f32,
    y: f32,
}

impl From<Sample> for Point {
    fn from(s: Sample) -> Self {
        Point { x: s[0], y: s[1] }
    }
}

#[get("/data?<dataset_id>")]
pub fn data(conn: DbConn, dataset_id: i32) -> Option<Json<APIResult>> {
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
