mod app;
mod listener;

use app::{AxumServer, AxumService};
use listener::{Listener, ListenerService};

pub trait ServerService {
    fn new(port: u16) -> Self;
    async fn start(self) -> Result<(), std::io::Error>;
}

pub struct Server {
    port: u16,
}

impl ServerService for Server {
    fn new(port: u16) -> Self {
        Self { port }
    }

    async fn start(self) -> Result<(), std::io::Error> {
        let listener: Listener = ListenerService::new(self.port);
        let tcp_listener = listener.listen().await;
        let axum: AxumServer = AxumService::new(tcp_listener);
        println!("Starting Axum server on port {}", self.port);
        axum.server().await
    }
}
