use crate::{
    features::price::use_cases::read_price::ReadPriceUseCaseTrait,
    traits::controller::{ApiResponse, ControllerTrait},
};
use axum::{response::IntoResponse, routing::get, Router};

pub trait ReadPriceHandlerTrait<U> {
    fn new(use_case: U) -> Self;
    async fn perform(self) -> impl IntoResponse;
}

#[derive(Clone)]
pub struct ReadPriceController<U> {
    read_price_use_case: U,
}

impl<U> ReadPriceHandlerTrait<U> for ReadPriceController<U>
where
    U: ReadPriceUseCaseTrait,
{
    fn new(use_case: U) -> Self {
        Self {
            read_price_use_case: use_case,
        }
    }

    async fn perform(self) -> impl IntoResponse {
        let data = self.read_price_use_case.read_price().await;
        let response: ApiResponse<u32, String> = ApiResponse {
            code: "200".to_string(),
            transaction: "read".to_string(),
            message: data,
            data: Some(100u32),
            args: None,
        };
        axum::Json(response)
    }
}

impl<U> ControllerTrait for ReadPriceController<U>
where
    U: ReadPriceUseCaseTrait + Send + Sync + Clone + 'static,
{
    fn handler(self) -> Router {
        Router::new().route("/read", get(move || async move { self.perform() }))
    }
}
