use crate::generation;

use rocket_contrib::json::Json;

use serde::Serialize;

#[derive(Serialize)]
pub struct Point {
    pub x: f32,
    pub y: f32
}

#[get("/data")]
pub fn data() -> Json<Vec<Point>> {
    let v = generation::generate_dataset(42);
    Json(v)
}
