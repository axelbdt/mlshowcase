#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;


use rocket_contrib::serve::StaticFiles;
use rocket_contrib::json::Json;
use serde::Serialize;

use rand::prelude::*;
use rand_distr::{Distribution, StandardNormal};
use rand::rngs::StdRng;

#[derive(Serialize)]
struct Point {
    x: f32,
    y: f32
}

#[get("/data")]
fn data() -> Json<Vec<Point>> {
    let mut rng = StdRng::seed_from_u64(42);

    let mu: [f32; 2] = [6.0, 5.0];
    let sigma: [[f32; 2]; 2] = [[2.0, 1.0], [1.0, 3.0]];
    
    // Cholesky decomposition
    let a = sigma[0][0].sqrt();
    let b = sigma[0][1] / a;
    let c = (sigma[1][1]).sqrt() - b * b;

    let sqrt_sigma = [ [a, 0.], [b, c]];

    let v: Vec<Point> = (0..100).map(|_| {
        let x0: f32 = rng.sample(StandardNormal);
        let y0: f32 = rng.sample(StandardNormal);
        let x = x0 * sqrt_sigma[0][0] + mu[0];
        let y = x0 * sqrt_sigma[1][0] + y0 * sqrt_sigma[1][1] + mu[1];
        Point {x, y}
    }).collect();

    Json(v)
}

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("static"))
        .mount("/api", routes![data])
        .launch();
}
