#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

// example:

#[get("/<name>")]
fn hello(name: String) -> String {
    format!("Hello {}!", name)
}


fn main() {
    rocket::ignite()
        .mount("/", routes![hello])
        .launch();
}