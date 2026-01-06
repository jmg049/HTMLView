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

    /// Version mismatch between library and viewer.
    #[error("version mismatch: library v{library}, viewer v{viewer}\n{suggestion}")]
    VersionMismatch {
        library: String,
        viewer: String,
        suggestion: String,
    },

    /// An I/O error occurred.
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// A serialization error occurred.
    #[error("serialization error: {0}")]
    SerdeError(String),

    /// Command timed out waiting for response.
    #[error("command timed out after {timeout_secs}s (seq: {seq})")]
    CommandTimeout { seq: u64, timeout_secs: u64 },

    /// Command execution failed.
    #[error("command failed: {0}")]
    CommandFailed(String),

    /// Refresh not supported (old viewer or wrong mode).
    #[error("refresh not supported: {0}")]
    RefreshNotSupported(String),
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
            ViewerError::VersionMismatch {
                library,
                viewer,
                suggestion,
            } => ViewerError::VersionMismatch {
                library: library.clone(),
                viewer: viewer.clone(),
                suggestion: suggestion.clone(),
            },
            ViewerError::IoError(err) => {
                ViewerError::IoError(std::io::Error::new(err.kind(), err.to_string()))
            }
            ViewerError::SerdeError(err) => ViewerError::SerdeError(err.to_string()),
            ViewerError::CommandTimeout { seq, timeout_secs } => ViewerError::CommandTimeout {
                seq: *seq,
                timeout_secs: *timeout_secs,
            },
            ViewerError::CommandFailed(err) => ViewerError::CommandFailed(err.clone()),
            ViewerError::RefreshNotSupported(err) => ViewerError::RefreshNotSupported(err.clone()),
        }
    }
}
