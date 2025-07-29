use tokio::net::TcpListener;

pub trait ListenerService {
    fn new(port: u16) -> Self;
    async fn listen(&self) -> TcpListener;
}

pub struct Listener {
    port: u16,
}

impl ListenerService for Listener {
    fn new(port: u16) -> Self {
        Self { port }
    }

    async fn listen(&self) -> TcpListener {
        TcpListener::bind(format!("0.0.0.0:{}", self.port))
            .await
            .expect("Failed to bind TCP listener")
    }
}
