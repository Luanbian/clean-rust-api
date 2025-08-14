use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{
    features::price::use_cases::read_price::ReadPriceUseCase, services::axum::ApiResponse,
};

pub struct ReadPriceController {
    read_price_use_case: Arc<ReadPriceUseCase>,
}

impl ReadPriceController {
    pub fn new(use_case: Arc<ReadPriceUseCase>) -> Self {
        Self {
            read_price_use_case: use_case,
        }
    }

    pub async fn handler(&self, id: i32) -> axum::Json<ApiResponse<String, String>> {
        match self.read_price_use_case.perform(id).await {
            Ok(price) => {
                let response: ApiResponse<String, String> = ApiResponse {
                    code: "clean.rust.api.price.read".to_string(),
                    transaction: "1234567890".to_string(),
                    message: "Price retrieved successfully".to_string(),
                    data: Some(price),
                    args: None,
                };
                axum::Json(response)
            }
            Err(err) => {
                let response: ApiResponse<String, String> = ApiResponse {
                    code: "clean.rust.api.price.read.error".to_string(),
                    transaction: "1234567890".to_string(),
                    message: "Error retrieving price".to_string(),
                    data: None,
                    args: Some(err.to_string()),
                };
                axum::Json(response)
            }
        }
    }

    pub fn router(self: Arc<Self>) -> Router {
        Router::new().route(
            "/price/{id}",
            get(move |axum::extract::Path(id)| {
                let controller = self.clone();
                async move { controller.handler(id).await }
            }),
        )
    }
}
