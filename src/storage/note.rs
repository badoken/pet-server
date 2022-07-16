use std::collections::HashMap;
use sea_query::Query;
use uuid::Uuid;
use crate::core::note::{Note, NoteId, NoteRepo};

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

pub struct NoteStorage {
    storage: HashMap<NoteId, Note>,
}

impl NoteStorage {
    pub fn new() -> NoteStorage {
        NoteStorage { storage: HashMap::new() }
    }
}

impl NoteRepo for NoteStorage {
    fn add(&mut self, note: Note) {
        let id = note.id.clone();
        self.storage.insert(id, note);
    }

    fn update(&mut self, note: Note) {
        todo!()
    }

    fn find_all(&mut self) -> Vec<Note> {
        todo!()
    }

    fn find_by_id(&mut self, id: NoteId) -> Option<Note> {
        todo!()
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

