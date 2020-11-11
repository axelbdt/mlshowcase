#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;


use rocket_contrib::serve::StaticFiles;
use rocket_contrib::json::Json;
use serde::Serialize;

use rand::thread_rng;
use rand_distr::{Distribution, Normal};

#[derive(Serialize)]
struct Point {
    x: f32,
    y: f32
}

#[get("/data")]
// fn data() -> Json<Vec<Point>> {
fn data() -> Json<Vec<Point>> {
    let mut rng = thread_rng();
    let normal_x = Normal::new(2.0, 3.0).unwrap();
    let normal_y = Normal::new(5.0, 1.0).unwrap();
    let v: Vec<Point> = (0..100).map(|_| {
        let x = normal_x.sample(&mut rng);
        let y = normal_y.sample(&mut rng);
        Point {x, y}
    }).collect();
    // let v = normal.sample(&mut rng);

    Json(v)
}

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("static"))
        .mount("/api", routes![data])
        .launch();
}
