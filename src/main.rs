#[macro_use]
extern crate rocket;

use std::fmt::format;
use rocket::futures::TryFutureExt;
use rocket::tokio;
use log::{error, info, warn};
use log4rs;
use tokio::fs::read_to_string;

use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Logger, Root};

#[get("/")]
fn index() -> &'static str {
    "try /param/{number}"
}

#[get("/param/<param>")]
fn parameterised(param: u64) -> String {
    format!("Nice! The param was {}", param)
}

#[get("/file/<path>")]
async fn file(path: String) -> String {
    read_to_string(path).await.unwrap_or("Failed to open:(".to_string())
}

#[tokio::main]
async fn main() -> Result<(), rocket::Error> {
    configure_logging();

    let _rocket = rocket::build()
        .mount("/", routes![index, parameterised, file])
        .launch()
        .await?;

    Ok(())
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
