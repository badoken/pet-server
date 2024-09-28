use std::sync::Arc;
use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;

use uuid::Uuid;
use crate::{NoteRepo};

#[derive(Debug, Clone, Serialize, Deserialize, PartialOrd, PartialEq, Hash)]
pub struct NoteId(pub Uuid);

impl Eq for NoteId {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialOrd, PartialEq, Hash)]
pub struct NoteName(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, PartialOrd, PartialEq, Hash)]
pub struct NoteContent(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, PartialOrd, PartialEq, Hash)]
pub struct Note {
    pub id: NoteId,
    pub name: NoteName,
    pub content: NoteContent,
}

pub struct AppState {
    pub note_repo: Arc<Mutex<NoteRepo>>,
}