use std::error::Error;

#[tokio::main]
async fn main() {
    let _ = app::run().await.unwrap();
}
