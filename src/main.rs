use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddrV4};
use app::AppConfig;
use app::storage::db::DbConfig;

#[tokio::main]
async fn main() {
    let config = AppConfig {
        port: 8080,
        db: DbConfig {
            address: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 5432),
            user: "postgres",
            pass: "postgres",
        },
    };
    let _ = app::run(config).await.unwrap();
}
