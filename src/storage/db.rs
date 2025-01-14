use diesel::{Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::net::SocketAddrV4;

pub struct DbConfig {
    pub address: SocketAddrV4,
    pub user: &'static str,
    pub pass: &'static str,
}

pub struct Db {
    pub connection: PgConnection,
}

impl Db {
    pub fn new(config: &DbConfig) -> Db {
        let mut connection = PgConnection::establish(&format!(
            "postgres://{}:{}@{}:{}",
            config.user,
            config.pass,
            config.address.ip().to_string(),
            config.address.port()
        ))
        .unwrap();

        let migrations: EmbeddedMigrations = embed_migrations!("rsc/db_schema");
        let applied_migrations = connection.run_pending_migrations(migrations).expect("Failed to run migrations");
        println!("Applied migrations: {:?}", applied_migrations);

        Db { connection }
    }
}