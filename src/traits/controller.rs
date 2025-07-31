pub trait ControllerTrait {
    fn handler(self) -> axum::Router;
}

#[derive(serde::Serialize)]
pub struct ApiResponse<T, E> {
    pub code: String,
    pub transaction: String,
    pub message: String,
    pub data: Option<T>,
    pub args: Option<E>,
}
