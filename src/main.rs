mod constants;

use constants::axum::get_axum_port;
use constants::load_env;

mod services;
use services::axum::{Server, ServerService};
use services::postgres::DatabasePool;

mod features;
use features::price::routes::{PriceRoutes, PriceRoutesTrait};

mod enums;

#[tokio::main]
async fn main() {
    load_env();

    let db = DatabasePool::new()
        .await
        .expect("Failed to connect to database");

    let port: u16 = get_axum_port();
    let server: Server = Server::new(port)
        .await
        .add_route(PriceRoutes::read_route(db));
    server.start().await.expect("Failed to start server");
}
