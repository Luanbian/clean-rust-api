use dotenv::dotenv;

pub mod postgres;

pub fn load_env() {
    dotenv().ok();
}
