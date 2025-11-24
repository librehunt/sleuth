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
    fn test_error_types_all() {
        let config_err = SleuthError::Config("config".to_string());
        assert!(config_err.to_string().contains("Configuration error"));

        let site_err = SleuthError::Site("site".to_string());
        assert!(site_err.to_string().contains("Site error"));

        let validation_err = SleuthError::Validation("validation".to_string());
        assert!(validation_err.to_string().contains("Validation error"));

        let unknown_err = SleuthError::Unknown("unknown".to_string());
        assert!(unknown_err.to_string().contains("Unknown error"));
    }

    #[test]
    fn test_result_type() {
        let result: Result<()> = Err(SleuthError::Validation("test".to_string()));
        assert!(result.is_err());
    }

    #[test]
    fn test_result_type_ok() {
        let result: Result<()> = Ok(());
        assert!(result.is_ok());
    }

    #[test]
    fn test_error_debug() {
        let err = SleuthError::Config("test".to_string());
        let debug_str = format!("{:?}", err);
        assert!(!debug_str.is_empty());
    }

    #[test]
    fn test_error_display() {
        let err = SleuthError::Unknown("test error".to_string());
        let display_str = err.to_string();
        assert!(display_str.contains("test error"));
    }
}
