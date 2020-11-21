#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;

extern crate nalgebra as na;

mod db;
mod error;
mod handlers;
mod models;
mod routes;
mod schema;

use rocket_contrib::serve::StaticFiles;

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", StaticFiles::from("static"))
        .mount(
            "/api",
            routes![
                routes::datasets::get_ids,
                routes::data::data,
                routes::kmeans::kmeans
            ],
        )
        .attach(db::DbConn::fairing())
}
