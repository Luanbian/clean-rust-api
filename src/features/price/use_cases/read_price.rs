use crate::{
    enums::use_case::{UseCaseError, UseCaseResult},
    features::price::repository::traits::PriceRepositoryTrait,
};
use std::sync::Arc;

pub struct ReadPriceUseCase {
    repository: Arc<dyn PriceRepositoryTrait>,
}

impl ReadPriceUseCase {
    pub fn new(repository: Arc<dyn PriceRepositoryTrait>) -> Self {
        Self { repository }
    }

    pub async fn perform(&self, input: i32) -> UseCaseResult<String> {
        let price = self.repository.find_by_id(input).await;
        match price {
            Ok(Some(price)) => Ok(format!("Price: {price}")),
            Ok(None) => Err(UseCaseError::NotFoundError {
                message: format!("Price with ID {input} not found"),
            }),
            Err(err) => Err(UseCaseError::InternalServerError {
                message: format!("Error retrieving price: {err}"),
            }),
        }
    }
}
