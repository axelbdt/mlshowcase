#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;

extern crate nalgebra as na;

mod data;
mod db;
mod error;
mod models;
mod routes;
mod schema;

use rocket_contrib::serve::StaticFiles;

#[database("mlshowcase")]
pub struct DbConn(diesel::PgConnection);

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", StaticFiles::from("static"))
        .mount("/api", routes![routes::data::data, routes::kmeans::kmeans])
        .attach(DbConn::fairing())
}
