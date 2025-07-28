use axum::{routing::get, Json, Router};
use serde_json::json;
use tokio::net::TcpListener;

pub async fn server(app: Router) {
    let app = health_check(app);
    let port = 3005;

    let listener = TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .expect("Failed to bind TCP listener");
    println!("Server running on port: {port}");
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

fn health_check(app: Router) -> Router {
    app.route("/api", get(|| async { Json(json!({"message": "OK"})) }))
}
