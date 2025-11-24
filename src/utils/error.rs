//! Error types and handling

use thiserror::Error;

/// Main error type for sleuth
#[derive(Error, Debug)]
pub enum SleuthError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Site error: {0}")]
    Site(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Result type alias for sleuth operations
pub type Result<T> = std::result::Result<T, SleuthError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_types() {
        let err = SleuthError::Config("test config error".to_string());
        assert!(err.to_string().contains("Configuration error"));
    }

    #[test]
    fn test_result_type() {
        let result: Result<()> = Err(SleuthError::Validation("test".to_string()));
        assert!(result.is_err());
    }
}
