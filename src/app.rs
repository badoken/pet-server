#[macro_use]
extern crate rocket;

use std::fmt::format;
use std::thread::sleep;
use std::time::Duration;

pub mod core;
pub mod web;
pub mod storage;

use web::*;

use rocket::futures::TryFutureExt;

use log::{error, info, warn};
use log4rs;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use rocket::{Error, Ignite, Rocket};
use rocket::response::Redirect;
use crate::storage::note::NoteStorage;

#[get("/")]
pub fn index() -> Redirect { Redirect::to(uri!(note::get_all)) }

#[get("/health")]
pub fn health() -> &'static str { "OK" }

pub async fn run() -> Result<Rocket<Ignite>, Error> {
    configure_logging();

    rocket::build()
        .mount("/", routes![health, index, note::get_by_id, note::get_all, note::post])
        .manage(AppState { note_repo: NoteStorage::new() })
        .launch()
        .await
}

struct AppState {
    note_repo: NoteStorage,
}

fn configure_logging() {
    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        // .logger(Logger::builder().build("app::backend::db", LevelFilter::Info))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).unwrap();
}