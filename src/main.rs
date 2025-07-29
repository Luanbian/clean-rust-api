mod services;

use services::axum::Server;

use crate::services::axum::ServerService;

#[tokio::main]
async fn main() {
    let port: u16 = 3005;
    let server: Server = Server::new(port);
    server.start().await.expect("Failed to start server");
}
