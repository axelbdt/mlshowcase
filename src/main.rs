use ml_showcase;

use serde::Serialize;


#[derive(Serialize)]
struct Point {
    x: f32,
    y: f32
}

fn wrap_point(p: (f32, f32)) -> Point {
    Point {x: p.0, y: p.1}
}


fn main() {
    ml_showcase::rocket().launch();
}
