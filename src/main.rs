mod constants;
use constants::axum::get_axum_port;
use constants::load_env;

mod services;
use services::axum::{Server, ServerService};
use services::postgres::{Database, DbService};

mod features;
use features::price::routes::PriceRoutes;

use crate::features::price::routes::PriceRoutesTrait;
mod traits;

#[tokio::main]
async fn main() {
    load_env();

    Database::connect_db()
        .await
        .expect("Failed to connect to the database");

    let port: u16 = get_axum_port();
    let server: Server = Server::new(port).await.add_route(PriceRoutes::read_route());
    server.start().await.expect("Failed to start server");
}
