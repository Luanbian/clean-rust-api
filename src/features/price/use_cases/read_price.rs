use std::sync::Arc;

use crate::traits::{repository::RepositoryTrait, use_cases::UseCaseTrait};

#[derive(Clone)]
pub struct ReadPriceUseCase {
    repository: Arc<dyn RepositoryTrait<Input = (), Output = i32> + Send + Sync>,
}

impl ReadPriceUseCase {
    pub fn new(
        repository: Arc<dyn RepositoryTrait<Input = (), Output = i32> + Send + Sync>,
    ) -> Self {
        Self { repository }
    }
}

impl UseCaseTrait for ReadPriceUseCase {
    type Input = ();
    type Output = String;

    fn perform(&self, _input: Option<Self::Input>) -> Self::Output {
        let price = self.repository.main(None);
        match price {
            Some(value) => format!("Price is: {value}"),
            None => "Price not found".to_string(),
        }
    }
}
