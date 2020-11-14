use crate::generation;
use crate::generation::{Data, Sample};

use rocket_contrib::json::Json;

use serde::Serialize;

#[derive(Serialize)]
pub struct Point {
    x: f32,
    y: f32
}

impl Point {
    fn from_sample(v: &Sample) -> Self {
        Point {
            x: v[0],
            y: v[1]
        }
    }

}

fn data_to_points(data: Data) -> Vec<Point> {
    data.iter()
        .map(Point::from_sample)
        .collect()
}

#[get("/data")]
pub fn data() -> Json<Vec<Point>> {
    let v = data_to_points(generation::generate_dataset(42));
    Json(v)
}
