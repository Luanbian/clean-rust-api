use sqlx::PgPool;

use crate::{constants::postgres::get_db_url, enums::repository::RepositoryError};

#[derive(Clone)]
pub struct DatabasePool {
    db_pool: PgPool,
}

impl DatabasePool {
    pub async fn new() -> Result<Self, RepositoryError> {
        let db_url = get_db_url();
        let pool = PgPool::connect(&db_url)
            .await
            .map_err(|err| RepositoryError::Connection {
                message: err.to_string(),
            })?;

        sqlx::query("SELECT 1")
            .execute(&pool)
            .await
            .map_err(|err| RepositoryError::TestQuery {
                message: err.to_string(),
            })?;

        println!("Connected to the database successfully.");

        Ok(Self { db_pool: pool })
    }

    pub fn get_pool(&self) -> &PgPool {
        &self.db_pool
    }
}
