#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

extern crate nalgebra as na;

mod generation;
mod processing;
mod routes;

use rocket_contrib::databases::diesel;
use rocket_contrib::serve::StaticFiles;

#[database("mlshowcase")]
struct DbConn(diesel::PgConnection);

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", StaticFiles::from("static"))
        .mount("/api", routes![routes::data, routes::kmeans])
        .attach(DbConn::fairing())
}
