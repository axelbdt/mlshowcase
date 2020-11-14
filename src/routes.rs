use crate::generation;

use rocket_contrib::json::Json;

use serde::Serialize;

#[derive(Serialize)]
pub struct Point {
    x: f32,
    y: f32
}

#[get("/data")]
pub fn data() -> Json<Vec<Point>> {
    let v = generation::generate_dataset(42)
        .iter()
        .map( |c| Point {x: c[0], y: c[1]})
        .collect();
    Json(v)
}
