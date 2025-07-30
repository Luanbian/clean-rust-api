use dotenv::dotenv;

pub mod axum;
pub mod postgres;

pub fn load_env() {
    dotenv().ok();
}
