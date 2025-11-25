use thiserror::Error;

/// Errors that can occur when using the html_view library.
#[derive(Error, Debug)]
pub enum ViewerError {
    /// The html_view_app binary could not be found.
    #[error("html_view_app binary not found: {0}")]
    BinaryNotFound(String),

    /// Failed to spawn the html_view_app process.
    #[error("failed to spawn html_view_app: {0}")]
    SpawnFailed(String),

    /// Failed to write the configuration file.
    #[error("failed to write configuration file: {0}")]
    ConfigWriteFailed(String),

    /// Failed to read the result file.
    #[error("failed to read result file: {0}")]
    ResultReadFailed(String),

    /// The response from the viewer was invalid or malformed.
    #[error("invalid response from viewer: {0}")]
    InvalidResponse(String),

    /// The viewer timed out.
    #[error("viewer timed out")]
    Timeout,

    /// An I/O error occurred.
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// A serialization error occurred.
    #[error("serialization error: {0}")]
    SerdeError(String),
}

impl Clone for ViewerError {
    fn clone(&self) -> Self {
        match self {
            ViewerError::BinaryNotFound(err) => ViewerError::BinaryNotFound(err.clone()),
            ViewerError::SpawnFailed(err) => ViewerError::SpawnFailed(err.clone()),
            ViewerError::ConfigWriteFailed(err) => ViewerError::ConfigWriteFailed(err.clone()),
            ViewerError::ResultReadFailed(err) => ViewerError::ResultReadFailed(err.clone()),
            ViewerError::InvalidResponse(err) => ViewerError::InvalidResponse(err.clone()),
            ViewerError::Timeout => ViewerError::Timeout,
            ViewerError::IoError(err) => {
                ViewerError::IoError(std::io::Error::new(err.kind(), err.to_string()))
            }
            ViewerError::SerdeError(err) => ViewerError::SerdeError(err.to_string()),
        }
    }
}
