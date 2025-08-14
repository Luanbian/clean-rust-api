use crate::enums::repository::RepositoryResult;
use async_trait::async_trait;

#[async_trait]
pub trait PriceRepositoryTrait: Send + Sync {
    async fn find_by_id(&self, id: i32) -> RepositoryResult<Option<i32>>;
}
