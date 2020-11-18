use crate::error::Error;
use crate::generation;
use crate::generation::{Data, Sample};

use crate::processing;

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

#[get("/data")]
pub fn data() -> Json<APIResult> {
    Json(match generation::generate_dataset(42) {
        Ok(data) => data.into(),
        Err(e) => e.into(),
    })
}

#[get("/kmeans")]
pub fn kmeans() -> Json<APIResult> {
    let data = match generation::generate_dataset(42) {
        Ok(data) => data,
        Err(e) => return Json(e.into()),
    };
    let v = processing::kmeans(3, &data);
    Json(v.into())
}
