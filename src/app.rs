#[macro_use]
extern crate rocket;

use std::fmt::format;
use std::sync::Arc;
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
use rocket::{Error, Ignite, Rocket, State};
use rocket::response::Redirect;
use rocket::serde::json::Json;
use tokio::sync::Mutex;
use crate::core::note::AppState;
use crate::storage::note::NoteRepo;

#[get("/")]
pub fn index() -> Redirect { Redirect::to(uri!(note::get_all)) }

#[get("/health")]
pub fn health(state: &State<AppState>) -> Json<()> { Json(()) }

pub async fn run() -> Result<Rocket<Ignite>, Error> {
    configure_logging();

    rocket::build()
        .manage(AppState { note_repo: Arc::new(Mutex::new(NoteRepo::new())) })
        .mount("/", routes![health, index, note::get_by_id, note::get_all, note::post])
        .launch()
        .await
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