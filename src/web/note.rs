use rocket::response::Redirect;
use tokio::fs::read_to_string;
use crate::core::note::NoteId;


#[get("/note/<note_id>")]
pub fn get_by_id(note_id: String) -> String {
    format!("Nice! The param was {:?}", note_id)
}

#[get("/notes")]
pub fn get_all() -> String {
    format!("TODO")
}

#[post("/note")]
pub fn post() -> String {
    format!("TODO")
}
