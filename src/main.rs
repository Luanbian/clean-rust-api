use axum::Router;

mod services;
use services::axum::server;

#[tokio::main]
async fn main() {
    let app = Router::new().nest("/api", Router::new());
    server(app).await;
}
