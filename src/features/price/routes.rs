use crate::{
    features::price::{
        controllers::read::ReadPriceController, repository::find_price_repo::FindPriceRepo,
        use_cases::read_price::ReadPriceUseCase,
    },
    services::postgres::DatabasePool,
};
use std::sync::Arc;

pub trait PriceRoutesTrait {
    fn read_route(db: DatabasePool) -> axum::Router;
}

pub struct PriceRoutes;

impl PriceRoutesTrait for PriceRoutes {
    fn read_route(db: DatabasePool) -> axum::Router {
        let repository = Arc::new(FindPriceRepo::new(Arc::new(db)));
        let use_case = ReadPriceUseCase::new(repository);
        let controlller = Arc::new(ReadPriceController::new(Arc::new(use_case)));
        controlller.router()
    }
}
