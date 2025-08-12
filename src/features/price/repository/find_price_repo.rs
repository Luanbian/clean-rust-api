use crate::{
    services::postgres::{Database, DbService},
    traits::repository::RepositoryTrait,
};
use sqlx::Row;

pub struct FindPriceRepo;

impl RepositoryTrait for FindPriceRepo {
    type Input = ();
    type Output = i32;

    async fn main(
        &self,
        _input: Option<Self::Input>,
    ) -> Result<Self::Output, Box<dyn std::error::Error>> {
        let db = Database::get_db().unwrap();
        let result = sqlx::query("SELECT price FROM product LIMIT 1")
            .fetch_one(db)
            .await?;
        let price = result.try_get("price")?;
        Ok(price)
    }
}
