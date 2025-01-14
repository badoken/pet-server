use diesel::{ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper};
use uuid::Uuid;

use crate::core::note::{Note, NoteId};
use crate::storage::db::Db;
use crate::storage::{db, schema};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = schema::notes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NoteRecord {
    pub id: Uuid,
    pub name: String,
    pub content: String,
}

pub struct NoteRepo {
    db: Db,
}
impl Into<Note> for NoteRecord {
    fn into(self) -> Note {
        Note {
            id: NoteId(self.id),
            name: self.name.into(),
            content: self.content.into(),
        }
    }
}

impl NoteRepo {
    pub async fn new(db: Db) -> NoteRepo {
        NoteRepo { db }
    }

    pub async fn add(&mut self, note: Note) -> Result<(), String> {
        let record = NoteRecord {
            id: note.id.0,
            name: note.name.into(),
            content: note.content.into(),
        };
        match diesel::insert_into(schema::notes::table)
            .values(record)
            .execute(&mut self.db.connection)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn update(&mut self, _note: Note) {
        todo!()
    }

    pub async fn find_all(&mut self) -> Result<Vec<Note>, String> {
        match schema::notes::table
            .select(NoteRecord::as_select())
            .load(&mut self.db.connection)
        {
            Ok(notes) => Ok(notes.into_iter().map(|n| n.into()).collect()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn find_by_id(&mut self, id: &NoteId) -> Result<Option<Note>, String> {
        match schema::notes::table
            .filter(schema::notes::id.eq(id.0))
            .limit(1)
            .select(NoteRecord::as_select())
            .load(&mut self.db.connection) {
            Ok(notes) => Ok(notes.into_iter().map(|n| n.into()).next()),
            Err(e) => Err(e.to_string()),
        }
    }
}
