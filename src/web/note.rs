use std::io::Cursor;
use rocket::data::{FromData, Outcome, ToByteUnit};
use rocket::request::FromParam;
use rocket::response::Redirect;
use rocket::{Data, Request, Response, State};
use rocket::http::Status;
use rocket::outcome::Outcome::{Failure, Success};
use rocket::serde::json::Json;
use serde_json::Value;
use tokio::fs::read_to_string;
use uuid::Uuid;
use crate::core::note::{AppState, Note, NoteId};


#[get("/note/<note_id>")]
pub async fn get_by_id(note_id: NoteId, state: &State<AppState>) -> Option<Json<Note>> {
    state.note_repo.lock().await.find_by_id(&note_id)
        .map(|n| Json(n))
}

#[get("/notes")]
pub async fn get_all(state: &State<AppState>) -> Json<Vec<Note>> {
    Json(state.note_repo.lock().await.find_all())
}

#[post("/note", data = "<note>")]
pub async fn post(note: Note, state: &State<AppState>) -> Json<()> {
    state.note_repo.lock().await.add(note);
    Json(())
}

impl<'r> FromParam<'r> for NoteId {
    type Error = uuid::Error;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        Uuid::parse_str(param).map(|id| NoteId(id))
    }
}

#[rocket::async_trait]
impl<'r> FromData<'r> for Note {
    type Error = String;

    async fn from_data(_req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        let data = data.open(256.bytes()).into_string().await;
        match data {
            Ok(string) =>
                match serde_json::from_str(string.as_str()) {
                    Ok(note) => Success(note),
                    Err(e) => Failure((Status::UnprocessableEntity, e.to_string()))
                },
            Err(e) => {
                error!("Failed to read data for a note:\n{:?}", e);
                Failure((Status::UnprocessableEntity, "Data read failure".to_string()))
            }
        }
    }
}