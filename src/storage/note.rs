use sea_query::{Expr, PostgresQueryBuilder, Query};
use tokio_postgres::{Row};
use uuid::Uuid;

use db::Database;

use crate::core::note::{NoteContent, NoteId, NoteName};
use crate::storage::db;

type CoreNote = crate::core::note::Note;

pub struct NoteRecord {
    pub id: Uuid,
    pub name: String,
    pub content: String,
}


#[derive(sea_query::Iden)]
enum Note {
    Table,
    Id,
    Name,
    Content,
}

pub struct NoteRepo {
    db: Database,
}

impl NoteRepo {
    pub async fn new(db: Database) -> NoteRepo {
        NoteRepo { db }
    }

    pub async fn add(&mut self, note: CoreNote) -> Result<(), String> {
        let statement = Query::insert()
            .into_table(Note::Table)
            .columns([Note::Id, Note::Name, Note::Content])
            .values(vec![note.id.0.to_string().into(), note.name.0.into(), note.content.0.into()])
            .map(|s| s.to_owned())
            .map(|s| s.to_string(PostgresQueryBuilder))
            .map_err(|e| e.to_string())?;

        self.db.insert(statement).await
    }

    pub fn update(&mut self, _note: CoreNote) {
        todo!()
    }

    pub async fn find_all(&self) -> Result<Vec<CoreNote>, String> {
        let statement = Query::select()
            .columns([Note::Id, Note::Name, Note::Content])
            .from(Note::Table)
            .to_owned();
        self.db.select(statement, |r| NoteRepo::note_from_row(r)).await
    }


    fn note_from_row(row: &Row) -> CoreNote { // FIXME
        CoreNote {
            id: NoteId(row.get(0)),
            name: NoteName(row.get(1)),
            content: NoteContent(row.get(2)),
        }
    }

    pub async fn find_by_id(&self, id: &NoteId) -> Result<Option<CoreNote>, String> {
        let statement = Query::select()
            .from(Note::Table)
            .columns([Note::Id, Note::Name, Note::Content])
            .and_where(Expr::col(Note::Id).eq(id.0.to_string()))
            .to_owned();
        let notes = self.db.select(statement, |r| NoteRepo::note_from_row(r)).await;

        match notes {
            Ok(notes) =>
                match &notes[..] {
                    [note] => Ok(Some(note.to_owned())),
                    [] => Ok(None),
                    _ => Err("Found more than one".to_string()),
                },
            Err(e) => Err(e)
        }
    }
}