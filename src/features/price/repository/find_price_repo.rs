use crate::{
    enums::repository::{RepositoryError, RepositoryResult},
    features::price::repository::traits::PriceRepositoryTrait,
    services::postgres::DatabasePool,
};
use async_trait::async_trait;
use sqlx::Row;
use std::sync::Arc;

pub struct FindPriceRepo {
    db: Arc<DatabasePool>,
}

impl FindPriceRepo {
    pub fn new(db: Arc<DatabasePool>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl PriceRepositoryTrait for FindPriceRepo {
    async fn find_by_id(&self, id: i32) -> RepositoryResult<Option<i32>> {
        let db = self.db.get_pool();
        let build_query = "SELECT price FROM products WHERE id = $1";
        let query = sqlx::query(build_query).bind(id);

        let row = query
            .fetch_optional(db)
            .await
            .map_err(|err| RepositoryError::Database {
                message: (err.to_string()),
            })?;

        Ok(row.map(|r| r.get::<i32, _>("price")))
    }
}
