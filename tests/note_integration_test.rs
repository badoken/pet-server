#[macro_use]
extern crate lazy_static;
extern crate core;
extern crate serde_json;

use std::collections::HashMap;
use std::error::Error;
use std::fmt::format;
use std::future::Future;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::ops::Add;
use std::pin::Pin;
use std::process::exit;
use std::sync::{Arc};
use std::sync::mpsc::{Receiver, Sender};
use std::time::{Duration, SystemTime};
use isahc::http::StatusCode;
use isahc::{AsyncReadResponseExt, get, get_async, post, post_async, ReadResponseExt, Request, RequestExt, Response};
use log::{error, info, warn};
use rocket::{Ignite, Rocket};
use serde_json::json;
use serde_json::value::Value;
use serde_json::value::Value::Array;
use tokio::sync::Mutex;
use uuid::Uuid;
use testcontainers::{clients, Container};
use testcontainers::clients::Cli;
use testcontainers::images::postgres::Postgres;
use app::AppConfig;
use app::storage::db::DbConfig;

lazy_static! {
    static ref SERVICE_IS_UP: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

#[tokio::test]
async fn should_add_and_read_a_note() {
    // given
    running_service().await;

    let id = Uuid::new_v4();
    let note = json!({
        "id": id.to_string(),
        "name": "note 1",
        "content": "some text\na new line"
    });

    // when
    let mut post_note = post_async("http://127.0.0.1:8080/note", note.clone().to_string()).await.unwrap();
    let mut get_note_by_id = get_async(format!("http://127.0.0.1:8080/note/{}", id)).await.unwrap();
    let mut get_notes = get_async("http://127.0.0.1:8080/notes").await.unwrap();

    // then
    assert_eq!(post_note.status(), StatusCode::OK);

    assert_eq!(get_notes.status(), StatusCode::OK);
    let actual_notes: Value = serde_json::from_str(get_notes.text().await.unwrap().as_str()).unwrap();
    assert_eq!(serde_json::Value::from(actual_notes), Array(vec![note.clone()]));

    assert_eq!(get_note_by_id.status(), StatusCode::OK);
    let actual_note: Value = serde_json::from_str(get_note_by_id.text().await.unwrap().as_str()).unwrap();
    assert_eq!(serde_json::value::to_value(actual_note).unwrap(), note.clone());
}

#[tokio::test]
async fn should_provide_no_note_if_the_id_is_unknown() -> Result<(), isahc::Error> {
    // given
    running_service().await;

    let id = Uuid::new_v4();

    // when
    let mut get_note_by_id = get_async(format!("http://127.0.0.1:8080/note/{}", id)).await?;

    // then
    assert_eq!(get_note_by_id.status(), StatusCode::NOT_FOUND);

    Ok(())
}

async fn running_service() -> () {
    let mut is_up = SERVICE_IS_UP.lock().await;
    if *is_up {
        return ();
    }


    let (pg_url_snd, pg_url_rcv) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let cli = clients::Cli::default();
        let (container, pg_url) = postgres(&cli);
        pg_url_snd.send(pg_url).unwrap();

        // TODO: Graceful closure
        loop {
            std::thread::sleep(Duration::from_secs(60));
        }
    });

    tokio::task::spawn(async move {
        let pg_url = pg_url_rcv.recv_timeout(Duration::from_secs(5)).unwrap();
        let config = AppConfig { port: 8080, db: DbConfig { address: pg_url, user: "postgres", pass: "" } };
        app::run(config).await.unwrap()
    });

    retry(|| {
        Box::pin(async {
            get_async("http://127.0.0.1:8080/health").await
                .map(|r| r.status())
                .ok()
                .filter(|s| StatusCode::OK.eq(s))
        })
    }).await;

    *is_up = true;

    return ();
}

fn postgres(docker_cli: &Cli) -> (Container<Postgres>, SocketAddrV4) {
    let container = docker_cli.run(Postgres::default());
    let port = container.get_host_port_ipv4(5432);

    let db_addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, port);

    (container, db_addr)
}

async fn retry<T, F>(until: F) -> T
    where F: Fn() -> Pin<Box<dyn Future<Output=Option<T>>>> {
    let start_time = SystemTime::now();
    loop {
        let result = until().await;
        match result {
            Some(t) => return t,
            None => {
                if SystemTime::now().gt(&start_time.add(Duration::from_secs(5))) {
                    panic!("Failed to wait");
                }
                tokio::time::sleep(Duration::from_micros(100)).await;
            }
        }
    }
}