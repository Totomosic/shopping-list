#![feature(plugin, decl_macro, proc_macro_hygiene)]
#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use dotenv::dotenv;
use std::env;
use std::process::Command;

use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};

mod auth;
mod db;
mod models;
mod responses;
mod routes;
mod schema;
mod utils;

fn rocket() -> rocket::Rocket {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");

    let pool = db::init_pool(database_url);

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    rocket::ignite()
        .manage(pool)
        .attach(cors.to_cors().unwrap())
        .mount("/", routes![routes::public::index])
        .mount(
            "/api/v1/",
            routes![
                routes::users::get_users,
                routes::auth::basic_auth,
                routes::auth::refresh_jwt
            ],
        )
}

fn main() {
    let _output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cd .. && yarn serve"])
            .spawn()
            .expect("Failed to start UI Application")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("cd .. && yarn serve")
            .spawn()
            .expect("Failed to start UI Application")
    };
    rocket().launch();
}
