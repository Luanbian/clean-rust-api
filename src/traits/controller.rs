use axum::response::IntoResponse;

pub trait ControllerTrait {
    type DataType;
    type ErrorType;

    async fn handler(&self) -> impl IntoResponse;

    async fn create(&self) -> ApiResponse<Self::DataType, Self::ErrorType> {
        ApiResponse {
            code: "501".to_string(),
            transaction: "create".to_string(),
            message: "Not Implemented".to_string(),
            data: None,
            args: None,
        }
    }

    async fn read(&self) -> ApiResponse<Self::DataType, Self::ErrorType> {
        ApiResponse {
            code: "501".to_string(),
            transaction: "read".to_string(),
            message: "Not Implemented".to_string(),
            data: None,
            args: None,
        }
    }

    async fn update(&self) -> ApiResponse<Self::DataType, Self::ErrorType> {
        ApiResponse {
            code: "501".to_string(),
            transaction: "update".to_string(),
            message: "Not Implemented".to_string(),
            data: None,
            args: None,
        }
    }

    async fn delete(&self) -> ApiResponse<Self::DataType, Self::ErrorType> {
        ApiResponse {
            code: "501".to_string(),
            transaction: "delete".to_string(),
            message: "Not Implemented".to_string(),
            data: None,
            args: None,
        }
    }
}

#[derive(serde::Serialize)]
pub struct ApiResponse<T, E> {
    pub code: String,
    pub transaction: String,
    pub message: String,
    pub data: Option<T>,
    pub args: Option<E>,
}
