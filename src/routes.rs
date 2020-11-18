use crate::generation;
use crate::generation::{Data, Sample};

use crate::processing;

use rocket_contrib::json::Json;

use serde::Serialize;

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

fn data_to_points(data: Data) -> Vec<Point> {
    data.into_iter().map(|s| s.into()).collect()
}

#[get("/data")]
pub fn data() -> Json<Vec<Point>> {
    let v = data_to_points(generation::generate_dataset(42));
    Json(v)
}

#[get("/kmeans")]
pub fn kmeans() -> Json<Vec<Point>> {
    let v = data_to_points(processing::kmeans(3));
    Json(v)
}
