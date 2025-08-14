use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Database error: {message}")]
    Database { message: String },

    #[error("Failed to connect to the database: {message}")]
    Connection { message: String },

    #[error("Failed to execute test query: {message}")]
    TestQuery { message: String },
}

pub type RepositoryResult<T> = Result<T, RepositoryError>;
