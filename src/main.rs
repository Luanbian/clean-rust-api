mod services;
use services::axum::{Server, ServerService};

#[tokio::main]
async fn main() {
    let port: u16 = 3005;
    let server: Server = Server::new(port);
    server.start().await.expect("Failed to start server");
}
