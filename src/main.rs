#[macro_use]
extern crate actix_web;

use std::{env, io};

use actix_web::{middleware, App, HttpServer};
use controllers::user_controller;

pub mod controllers;
pub mod factories;
pub mod models;
pub mod repositories;
pub mod services;
pub mod database;
pub mod id;


#[actix_rt::main]
async fn main() -> io::Result<()> {

    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(user_controller::create)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}

