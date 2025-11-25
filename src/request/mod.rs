//! Request abstraction for making HTTP requests

pub mod browser;
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
    Browser,
    Tor,
}

/// Factory for creating request implementations
pub fn create_request(request_type: RequestType, timeout_secs: u64) -> Result<Arc<dyn Request>> {
    match request_type {
        RequestType::Http => Ok(Arc::new(http::HttpRequest::new(timeout_secs)?)),
        RequestType::Browser => Ok(Arc::new(browser::BrowserRequest::new(timeout_secs)?)),
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
    fn test_create_http_request_zero_timeout() {
        let request = create_request(RequestType::Http, 0);
        assert!(request.is_ok());
    }

    #[test]
    fn test_create_http_request_large_timeout() {
        let request = create_request(RequestType::Http, 3600);
        assert!(request.is_ok());
    }

    #[test]
    fn test_create_tor_request() {
        let request = create_request(RequestType::Tor, 10);
        assert!(request.is_err()); // Not yet implemented
        if let Err(err) = request {
            assert!(err.to_string().contains("Tor not yet implemented"));
        }
    }

    #[test]
    fn test_request_type_debug() {
        let http_type = RequestType::Http;
        let debug_str = format!("{:?}", http_type);
        assert_eq!(debug_str, "Http");

        let tor_type = RequestType::Tor;
        let debug_str = format!("{:?}", tor_type);
        assert_eq!(debug_str, "Tor");
    }

    #[test]
    fn test_request_type_clone() {
        let http_type = RequestType::Http;
        let cloned = http_type;
        // Should compile (Copy trait)
        assert!(matches!(cloned, RequestType::Http));
    }
}
