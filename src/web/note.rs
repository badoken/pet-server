use rocket::data::{FromData, Outcome, ToByteUnit};
use rocket::request::FromParam;
use rocket::{Data, Request, State};
use rocket::http::Status;
use rocket::outcome::Outcome::{Error, Success};
use rocket::serde::json::Json;
use uuid::Uuid;
use crate::core::note::{AppState, Note, NoteContent, NoteId, NoteName};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NewNoteParams {
    pub name: String,
    pub content: String,
}



#[get("/note/<note_id>")]
pub async fn get_by_id(note_id: NoteId, state: &State<AppState>) -> Result<Json<Note>, (Status, String)> {
    match state.note_repo.lock().await.find_by_id(&note_id).await {
        Ok(Some(note)) => Ok(Json(note)),
        Ok(None) => Err((Status::NotFound, "Not found".to_string())),
        Err(e) => Err((Status::InternalServerError, e))
    }
}

#[get("/notes")]
pub async fn get_all(state: &State<AppState>) -> Result<Json<Vec<Note>>, String> {
    state.note_repo.lock().await
        .find_all().await
        .map(|notes| Json(notes))
}

#[post("/note", data = "<params>")]
pub async fn post(params: NewNoteParams, state: &State<AppState>) -> Result<Json<()>, String> {
    state.note_repo.lock().await
        .add(params.into())
        .await
        .map(|_| Json(()))
}

impl<'r> FromParam<'r> for NoteId {
    type Error = uuid::Error;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        Uuid::parse_str(param).map(|id| NoteId(id))
    }
}

#[rocket::async_trait]
impl<'r> FromData<'r> for NewNoteParams {
    type Error = String;

    async fn from_data(_req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        let maybe_string = data.open(256.bytes()).into_string().await;
        match maybe_string {
            Err(e) => Error((Status::UnprocessableEntity, e.to_string())),
            Ok(string) =>
                serde_json::from_str(string.as_str())
                    .map(|n| Success(n))
                    .unwrap_or_else(|e| Error((Status::UnprocessableEntity, e.to_string())))
        }
    }
}

impl Into<Note> for NewNoteParams {
    fn into(self) -> Note {
        Note {
            id: NoteId(Uuid::new_v4()),
            name: NoteName(self.name),
            content: NoteContent(self.content),
        }
    }
}
