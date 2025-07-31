use crate::features::price::{
    controllers::read::{ReadPriceController, ReadPriceHandlerTrait},
    repository::find_price_repo::FindPriceRepo,
    use_cases::read_price::{ReadPriceUseCase, ReadPriceUseCaseTrait},
};
use crate::traits::controller::ControllerTrait;
use std::sync::Arc;

pub trait PriceRoutesTrait {
    fn read_route() -> axum::Router;
}

pub struct PriceRoutes;

impl PriceRoutesTrait for PriceRoutes {
    fn read_route() -> axum::Router {
        let repository = Arc::new(FindPriceRepo);
        let use_case = ReadPriceUseCase::new(repository);
        let controlller = ReadPriceController::new(use_case);
        controlller.handler()
    }
}
