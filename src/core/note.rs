use std::fmt::Formatter;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::{EnumAccess, Error, MapAccess, SeqAccess, Visitor};

use uuid::Uuid;

#[derive(Debug, Clone, PartialOrd, PartialEq, Hash)]
pub struct NoteId(Uuid);

impl Eq for NoteId {}

impl Serialize for NoteId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(&self.0.to_string())
    }
}

struct NoteIdVisitor;

impl<'de> Visitor<'de> for NoteIdVisitor {
    type Value = NoteId;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result { write!(formatter, "UUID value") }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        Uuid::parse_str(v)
            .map(|uuid| NoteId(uuid))
            .map_err(|e| E::custom(e.to_string()))
    }
}

impl<'de> Deserialize<'de> for NoteId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_str(NoteIdVisitor)
    }
}

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

pub trait NoteRepo {
    fn add(&mut self, note: Note);
    fn update(&mut self, note: Note);
    fn find_all(&mut self) -> Vec<Note>;
    fn find_by_id(&mut self, id: NoteId) -> Option<Note>;
}