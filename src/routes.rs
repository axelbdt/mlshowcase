use crate::generation;

use rocket_contrib::json::Json;

use serde::Serialize;

#[derive(Serialize)]
pub struct Point {
    x: f32,
    y: f32
}

impl Point {
    fn from_array(p: &[f32;2]) -> Point {
        Point {
            x: p[0],
            y: p[1]
        }
    }
}

#[get("/data")]
pub fn data() -> Json<Vec<Point>> {

    let v = generation::generate_dataset(42)
        .iter().map(Point::from_array).collect();

    Json(v)
}
