mod app;
mod listener;

use app::{AxumServer, AxumService};
use listener::{Listener, ListenerService};

use crate::traits::controller::ControllerTrait;

pub trait ServerService {
    async fn new(port: u16) -> Self;
    async fn start(self) -> Result<(), std::io::Error>;
    fn add_route<T: ControllerTrait>(self, controller: T) -> Self;
}

pub struct Server {
    port: u16,
    http_server: AxumServer,
}

impl ServerService for Server {
    async fn new(port: u16) -> Self {
        let listener: Listener = ListenerService::new(port);
        let tcp_listener = listener.listen().await;

        Self {
            port,
            http_server: AxumServer::new(tcp_listener),
        }
    }

    async fn start(self) -> Result<(), std::io::Error> {
        println!("Starting Axum server on port {}", self.port);
        self.http_server.server().await
    }

    fn add_route<T: ControllerTrait>(mut self, controller: T) -> Self {
        self.http_server = self.http_server.add_route(controller);
        self
    }
}
