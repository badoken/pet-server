use std::borrow::BorrowMut;
use std::net::SocketAddrV4;
use std::ops::DerefMut;
use std::os::linux::raw::stat;
use std::sync::Arc;
use std::time::Duration;
use rocket::futures::StreamExt;
use rocket::http::ext::IntoCollection;
use sea_query::{PostgresQueryBuilder, SelectStatement};
use tokio::spawn;
use tokio::sync::Mutex;
use tokio_postgres::{Client, Error, NoTls, Row};

pub struct DbConfig {
    pub address: SocketAddrV4,
    pub user: &'static str,
    pub pass: &'static str,
}

pub struct Database {
    client: Arc<Mutex<Client>>,
}

impl Database {
    pub async fn new(config: &DbConfig) -> Database {
        let (mut client, connection) = tokio_postgres::Config::new()
            .host(config.address.ip().to_string().as_str())
            .port(config.address.port())
            .user(config.user)
            .password(config.pass)
            .connect(NoTls)
            .await
            .unwrap();

        spawn(async move { connection.await.unwrap(); });

        embedded_migrations::migrations::runner().run_async(&mut client).await.unwrap();

        Database { client: Arc::new(Mutex::new(client)) }
    }

    pub async fn select<F, T>(&self, statement: String, mapping: F) -> Result<Vec<T>, String>
        where F: Fn(&Row) -> T,
              T: Clone {
        // let raw_query = statement.to_string(PostgresQueryBuilder); // FIXME
        let client = self.client.lock().await;
        let result = client.query(statement.as_str(), &[]).await;
        match result {
            Err(e) => Err(e.to_string()),
            Ok(rows) => Ok(rows.iter().map(|r| mapping(&r)).collect())
        }
    }

    pub async fn insert(&self, statement: String) -> Result<(), String> {
        let client = self.client.lock().await;
        match client.execute(statement.as_str(), &[]).await {
            Ok(0) => Err(format!("Inserted no rows for {:?}", statement)),
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string())
        }
    }
}

mod embedded_migrations {
    use refinery::embed_migrations;
    embed_migrations!("./rsc/db_schema");
}