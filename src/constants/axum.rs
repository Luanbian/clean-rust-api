pub fn get_axum_port() -> u16 {
    std::env::var("PORT")
        .unwrap_or_else(|_| "3005".to_string())
        .parse::<u16>()
        .unwrap_or(3005)
}
