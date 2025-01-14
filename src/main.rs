use std::net::{Ipv4Addr, SocketAddrV4};
use app::AppConfig;
use app::storage::db::DbConfig;

#[tokio::main]
async fn main() {
    let config = AppConfig {
        port: 8080,
        db: DbConfig {
            address: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 5432),
            user: "postgres",
            pass: "pass",
        },
    };
    let _ = app::run(config).await.unwrap();
}
