use thiserror::Error;

#[derive(Error, Debug)]
pub enum UseCaseError {
    #[error("Record not found: {message}")]
    NotFoundError { message: String },

    #[error("Internal server error: {message}")]
    InternalServerError { message: String },
}

pub type UseCaseResult<T> = Result<T, UseCaseError>;
