mod constants;
use constants::load_env;

mod services;
use services::axum::{Server, ServerService};
use services::postgres::{Database, DbService};

mod features;
mod traits;

#[tokio::main]
async fn main() {
    load_env();

    Database::connect_db()
        .await
        .expect("Failed to connect to the database");

    let port: u16 = 3005;
    let server: Server = Server::new(port);
    server.start().await.expect("Failed to start server");
}
