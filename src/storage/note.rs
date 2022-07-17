use std::collections::HashMap;
use sea_query::Query;
use uuid::Uuid;
use crate::core::note::{Note, NoteId};

pub struct NoteRecord {
    pub id: Uuid,
    pub name: String,
    pub content: String,
}


#[derive(sea_query::Iden)]
enum NoteIden {
    Table,
    Id,
    Name,
    Content,
}

pub struct NoteRepo {
    storage: HashMap<NoteId, Note>,
}

impl NoteRepo {
    pub fn new() -> NoteRepo {
        NoteRepo { storage: HashMap::new() }
    }

    pub fn add(&mut self, note: Note) {
        let id = note.id.clone();
        self.storage.insert(id, note);
    }

    pub fn update(&mut self, note: Note) {
        todo!()
    }

    pub fn find_all(&self) -> Vec<Note> {
        self.storage.values().cloned().collect()
    }

    pub fn find_by_id(&self, id: &NoteId) -> Option<Note> {
        self.storage.get(id).cloned()
    }
}

fn store_record(record: NoteRecord) -> Result<Uuid, ()> {
    // Query::insert()
    //     .into_table(NoteIden::Table)
    //     .columns([NoteIden::Id, NoteIden::Name, NoteIden::Content])
    //     .values(vec![record.id, record.name, record.content])
    return todo!();
}


fn find_record(id: Uuid) -> Option<NoteRecord> {
    todo!();
    // return Query::select()
    //     .columns("id")
}

