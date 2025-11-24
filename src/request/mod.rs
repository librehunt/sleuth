//! Request abstraction for making HTTP requests

pub mod http;
pub mod tor;
pub mod trait_impl;

pub use trait_impl::{Request, RequestResponse};

use crate::utils::error::Result;
use std::sync::Arc;

/// Request type enum
#[derive(Debug, Clone, Copy)]
pub enum RequestType {
    Http,
    Tor,
}

/// Factory for creating request implementations
pub fn create_request(request_type: RequestType, timeout_secs: u64) -> Result<Arc<dyn Request>> {
    match request_type {
        RequestType::Http => Ok(Arc::new(http::HttpRequest::new(timeout_secs)?)),
        RequestType::Tor => {
            // TODO: Implement Tor
            Err(crate::utils::error::SleuthError::Unknown(
                "Tor not yet implemented".to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_http_request() {
        let request = create_request(RequestType::Http, 10);
        assert!(request.is_ok());
    }

    #[test]
    fn test_create_tor_request() {
        let request = create_request(RequestType::Tor, 10);
        assert!(request.is_err()); // Not yet implemented
    }
}
