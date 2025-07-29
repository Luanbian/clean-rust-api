use axum::{response::IntoResponse, Json};

use crate::traits::controller::{ApiResponse, ControllerTrait};

struct ReadPrice;

impl ControllerTrait for ReadPrice {
    type DataType = u32;
    type ErrorType = String;

    async fn handler(&self) -> impl IntoResponse {
        let response = self.read().await;
        Json(response)
    }

    async fn read(&self) -> ApiResponse<Self::DataType, Self::ErrorType> {
        ApiResponse {
            code: "200".to_string(),
            transaction: "read".to_string(),
            message: "Price read successfully".to_string(),
            data: Some(100u32),
            args: None,
        }
    }
}
