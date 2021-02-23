#[macro_use]
extern crate actix_web;
extern crate derive_new;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;

use std::env;

use actix_web::{guard, web, App, HttpResponse, HttpServer};
use anyhow::Context;

mod config;
mod controllers;
mod error;
mod middlewares;
mod models;
mod schema;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let app_host = env::var("APP_HOST").context("APP_HOST is not defined.")?;
    let app_port = env::var("APP_PORT").context("APP_PORT is not defined.")?;
    let db_url = env::var("DATABASE_URL").context("DATABASE_URL is not defined.")?;

    let pool = config::database::config_database(&db_url);

    let server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(actix_web::middleware::Logger::default())
            .wrap(config::error_handler::config_error_handlers())
            .configure(config::route::config_routes)
            .default_service(
                web::route()
                    .guard(guard::Not(guard::Get()))
                    .to(|| HttpResponse::MethodNotAllowed()),
            )
    })
    .bind(format!("{}:{}", app_host, app_port))
    .context("ポートのバインドに失敗しました。")?;

    server.run().await?;

    Ok(())
}
