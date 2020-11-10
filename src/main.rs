#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Serialize)]
struct Point {
    x: i32,
    y: i32
}

#[get("/data")]
fn data() -> Json<Vec<Point>> {
    let my_vec = vec![
            Point {x:-10, y:10},
            Point {x:0, y:10},
            Point {x:10, y:5},
        ];

    Json(my_vec)
}

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("static"))
        .mount("/api", routes![data])
        .launch();
}
