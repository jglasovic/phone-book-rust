#![feature(plugin, const_fn, decl_macro, proc_macro_hygiene)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use dotenv::dotenv;
use routes::*;
use std::env;

mod db;
mod models;
mod routes;
mod schema;

fn rocket() -> rocket::Rocket {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");

  let pool = db::init_pool(database_url);
  rocket::ignite()
    .manage(pool)
    .mount("/api/v1/", routes![get_all, add_new, find_single])
}

fn main() {
  rocket().launch();
}
