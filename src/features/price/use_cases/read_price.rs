use std::sync::Arc;

use crate::traits::repository::RepositoryTrait;

pub trait ReadPriceUseCaseTrait {
    fn new(repository: Arc<dyn RepositoryTrait>) -> Self;
    async fn read_price(&self) -> String;
}

#[derive(Clone)]
pub struct ReadPriceUseCase {
    repository: Arc<dyn RepositoryTrait>,
}

impl ReadPriceUseCaseTrait for ReadPriceUseCase {
    fn new(repository: Arc<dyn RepositoryTrait>) -> Self {
        Self { repository }
    }

    async fn read_price(&self) -> String {
        let price = self.repository.main();
        match price {
            Some(value) => format!("Price is: {value}"),
            None => "Price not found".to_string(),
        }
    }
}
