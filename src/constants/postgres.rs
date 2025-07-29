pub fn get_db_url() -> String {
    std::env::var("POSTGRES_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/postgres".to_string())
}
