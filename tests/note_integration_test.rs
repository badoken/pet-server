#[macro_use]
extern crate lazy_static;
extern crate core;
extern crate serde_json;

use std::error::Error;
use std::fmt::format;
use std::future::Future;
use std::ops::Add;
use std::pin::Pin;
use std::process::exit;
use std::sync::{Arc};
use std::time::{Duration, SystemTime};
use isahc::http::StatusCode;
use isahc::{AsyncReadResponseExt, get, get_async, post, post_async, ReadResponseExt, Request, RequestExt, Response};
use log::{error, warn};
use rocket::{Ignite, Rocket};
use serde_json::json;
use serde_json::value::Value;
use serde_json::value::Value::Array;
use tokio::sync::Mutex;
use uuid::Uuid;

lazy_static! {
    static ref SERVICE_IS_UP: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

async fn running_service() -> () {
    let mut is_up = SERVICE_IS_UP.lock().await;
    if *is_up {
        return ();
    }
    tokio::task::spawn(async { app::run().await.unwrap() });

    until_some(|| {
        Box::pin(async {
            get_async("http://127.0.0.1:8000/health").await
                .map(|r| r.status())
                .ok()
                .filter(|s| StatusCode::OK.eq(s))
        })
    }).await;

    *is_up = true;

    return ();
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
    let mut post_note = post_async("http://127.0.0.1:8000/note", note.clone().to_string()).await.unwrap();
    let mut get_note_by_id = get_async(format!("http://127.0.0.1:8000/note/{}", id)).await.unwrap();
    let mut get_notes = get_async("http://127.0.0.1:8000/notes").await.unwrap();

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
    let mut get_note_by_id = get_async(format!("http://127.0.0.1:8000/note/{}", id)).await?;

    // then
    assert_eq!(get_note_by_id.status(), StatusCode::NOT_FOUND);

    Ok(())
}

async fn until_some<T, F>(until: F) -> T
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