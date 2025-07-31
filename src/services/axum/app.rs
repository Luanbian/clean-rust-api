use axum::{routing::get, Router};
use tokio::net::TcpListener;

pub trait AxumService {
    fn new(listener: TcpListener) -> Self;
    async fn server(self) -> Result<(), std::io::Error>;
    async fn health_check(&self) -> Router;
    fn add_route(self, route: Router) -> Self;
}

pub struct AxumServer {
    listener: TcpListener,
    app: Router,
}

impl AxumService for AxumServer {
    fn new(listener: TcpListener) -> Self {
        Self {
            listener,
            app: Router::new(),
        }
    }

    async fn server(self) -> Result<(), std::io::Error> {
        let health_check = self.health_check().await;
        let app = self.app.nest("/api", health_check);

        axum::serve(self.listener, app).await
    }

    async fn health_check(&self) -> Router {
        axum::Router::new().route(
            "/",
            get(|| async { axum::Json(serde_json::json!({"message": "OK"})) }),
        )
    }

    fn add_route(mut self, route: Router) -> Self {
        self.app = self.app.merge(route);
        self
    }
}
