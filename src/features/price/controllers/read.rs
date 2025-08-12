use std::sync::Arc;

use crate::traits::{
    controller::{ApiResponse, ControllerTrait},
    use_cases::UseCaseTrait,
};
use axum::{response::IntoResponse, routing::get, Extension, Router};

pub trait ReadPriceControllerTrait {
    fn new(use_case: Box<dyn UseCaseTrait<Input = (), Output = String> + Send + Sync>) -> Self;
    async fn perform(&self) -> axum::Json<ApiResponse<u32, String>>;
}

pub struct ReadPriceController {
    read_price_use_case: Box<dyn UseCaseTrait<Input = (), Output = String> + Send + Sync>,
}

impl ReadPriceControllerTrait for ReadPriceController {
    fn new(use_case: Box<dyn UseCaseTrait<Input = (), Output = String> + Send + Sync>) -> Self {
        Self {
            read_price_use_case: use_case,
        }
    }

    async fn perform(&self) -> axum::Json<ApiResponse<u32, String>> {
        let data = &self.read_price_use_case.perform(None);
        let response: ApiResponse<u32, String> = ApiResponse {
            code: "200".to_string(),
            transaction: "read".to_string(),
            message: data.to_string(),
            data: Some(100u32),
            args: None,
        };
        axum::Json(response)
    }
}

async fn read_price_handler(
    Extension(controller): Extension<Arc<ReadPriceController>>,
) -> impl IntoResponse {
    controller.perform().await
}

impl ControllerTrait for ReadPriceController {
    fn handler(self) -> Router {
        let controller = Arc::new(self);
        Router::new()
            .route("/read", get(read_price_handler))
            .layer(Extension(controller))
    }
}
