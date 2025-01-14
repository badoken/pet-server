use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::NoteRepo;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialOrd, PartialEq, Hash)]
pub struct NoteId(pub Uuid);

impl Eq for NoteId {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialOrd, PartialEq, Hash)]
pub struct NoteName(pub String);
impl Into<String> for NoteName {
    fn into(self) -> String {
        self.0
    }
}

impl Into<NoteName> for String {
    fn into(self) -> NoteName {
        NoteName(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialOrd, PartialEq, Hash)]
pub struct NoteContent(pub String);
impl Into<String> for NoteContent {
    fn into(self) -> String {
        self.0
    }
}

impl Into<NoteContent> for String {
    fn into(self) -> NoteContent {
        NoteContent(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialOrd, PartialEq, Hash)]
pub struct Note {
    pub id: NoteId,
    pub name: NoteName,
    pub content: NoteContent,
}

pub struct AppState {
    pub note_repo: Arc<Mutex<NoteRepo>>,
}
