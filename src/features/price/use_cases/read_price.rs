use std::sync::Arc;

use crate::traits::{repository::RepositoryTrait, use_cases::UseCaseTrait};

#[derive(Clone)]
pub struct ReadPriceUseCase {
    repository: Arc<dyn RepositoryTrait>,
}

impl ReadPriceUseCase {
    pub fn new(repository: Arc<dyn RepositoryTrait>) -> Self {
        Self { repository }
    }
}

impl UseCaseTrait for ReadPriceUseCase {
    type Input = ();
    type Output = String;

    fn perform(&self, _input: Option<Self::Input>) -> Self::Output {
        let price = self.repository.main();
        match price {
            Some(value) => format!("Price is: {value}"),
            None => "Price not found".to_string(),
        }
    }
}
