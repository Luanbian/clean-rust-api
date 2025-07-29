use axum::{routing::get, Router};
use tokio::net::TcpListener;

pub trait AxumService {
    fn new(listener: TcpListener) -> Self;
    async fn server(self) -> Result<(), std::io::Error>;
    async fn health_check(&self) -> Router;
}

pub struct AxumServer {
    listener: TcpListener,
}

impl AxumService for AxumServer {
    fn new(listener: TcpListener) -> Self {
        Self { listener }
    }

    async fn server(self) -> Result<(), std::io::Error> {
        let health_check = self.health_check().await;
        let app = axum::Router::new().nest("/api", health_check);

        axum::serve(self.listener, app).await
    }

    async fn health_check(&self) -> Router {
        axum::Router::new().route(
            "/",
            get(|| async { axum::Json(serde_json::json!({"message": "OK"})) }),
        )
    }
}
