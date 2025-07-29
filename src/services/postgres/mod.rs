use sqlx::{Pool, Postgres};
use tokio::sync::OnceCell;

use crate::constants::postgres::get_db_url;

pub static DB: OnceCell<Pool<Postgres>> = OnceCell::const_new();

pub trait DbService {
    async fn connect_db() -> Result<(), sqlx::Error>;
    fn get_db() -> Result<&'static Pool<Postgres>, &'static str>;
}

pub struct Database;

impl DbService for Database {
    async fn connect_db() -> Result<(), sqlx::Error> {
        let pool = sqlx::postgres::PgPool::connect(&get_db_url()).await?;
        println!("Connected to the database successfully");
        DB.set(pool).map_err(|_| sqlx::Error::PoolTimedOut)?;
        Ok(())
    }

    fn get_db() -> Result<&'static Pool<Postgres>, &'static str> {
        DB.get().ok_or("Database not connected")
    }
}
