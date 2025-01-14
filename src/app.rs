#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel_migrations;

use std::sync::Arc;

pub mod core;
pub mod web;
pub mod storage;

use web::*;

use log4rs;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use rocket::{Config, Error, Ignite, Rocket};
use rocket::response::Redirect;
use rocket::serde::json::Json;
use tokio::sync::Mutex;
use crate::core::note::AppState;
use crate::storage::db::{Db, DbConfig};
use crate::storage::note::NoteRepo;

pub struct AppConfig {
    pub port: u16,
    pub db: DbConfig,
}

pub async fn run(conf: AppConfig) -> Result<Rocket<Ignite>, Error> {
    configure_logging();

    let rocket_config = Config { port: conf.port, ..Config::default() };
    let database = Db::new(&conf.db);


    rocket::build()
        .manage(AppState { note_repo: Arc::new(Mutex::new(NoteRepo::new(database).await)) })
        .mount("/", routes![health, index, note::get_by_id, note::get_all, note::post])
        .configure(&rocket_config)
        .launch()
        .await
}

#[get("/")]
pub fn index() -> Redirect { Redirect::to(uri!(note::get_all)) }

#[get("/health")]
pub fn health() -> Json<()> { Json(()) }


fn configure_logging() {
    let stdout = ConsoleAppender::builder().build();
    let config = log4rs::config::Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        // .logger(Logger::builder().build("app::backend::db", LevelFilter::Info))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).unwrap();
}