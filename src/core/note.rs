use std::fmt::Formatter;
use std::pin::Pin;
use std::sync::Arc;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::{EnumAccess, Error, MapAccess, SeqAccess, Visitor};
use tokio::sync::Mutex;

use uuid::Uuid;
use crate::{NoteRepo, storage};

#[derive(Debug, Clone, Serialize, Deserialize, PartialOrd, PartialEq, Hash)]
pub struct NoteId(pub Uuid);

impl Eq for NoteId {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialOrd, PartialEq, Hash)]
pub struct NoteName(String);

#[derive(Debug, Clone, Serialize, Deserialize, PartialOrd, PartialEq, Hash)]
pub struct NoteContent(String);

#[derive(Debug, Clone, Serialize, Deserialize, PartialOrd, PartialEq, Hash)]
pub struct Note {
    pub id: NoteId,
    pub name: NoteName,
    pub content: NoteContent,
}

pub struct AppState {
    pub note_repo: Arc<Mutex<NoteRepo>>,
}