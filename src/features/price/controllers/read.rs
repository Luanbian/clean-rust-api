use axum::{response::IntoResponse, routing::get, Router};

use crate::traits::controller::{ApiResponse, ControllerTrait};

pub struct ReadPrice;

impl ControllerTrait for ReadPrice {
    fn handler(&self) -> Router {
        Router::new().route("/read", get(read))
    }
}

async fn read() -> impl IntoResponse {
    let response: ApiResponse<u32, String> = ApiResponse {
        code: "200".to_string(),
        transaction: "read".to_string(),
        message: "Price read successfully".to_string(),
        data: Some(100u32),
        args: None,
    };
    axum::Json(response)
}
