//! Request trait definition

use crate::utils::error::Result;
use async_trait::async_trait;

/// Response from a request
#[derive(Debug, Clone)]
pub struct RequestResponse {
    pub status_code: u16,
    pub body: Option<String>,
    pub headers: Vec<(String, String)>,
}

impl RequestResponse {
    pub fn new(status_code: u16) -> Self {
        Self {
            status_code,
            body: None,
            headers: vec![],
        }
    }

    pub fn with_body(status_code: u16, body: String) -> Self {
        Self {
            status_code,
            body: Some(body),
            headers: vec![],
        }
    }
}

/// Trait for making HTTP requests
/// Implementations can be HTTP, Tor, or any other transport
#[async_trait]
pub trait Request: Send + Sync {
    /// Make a HEAD request
    async fn head(&self, url: &str) -> Result<RequestResponse>;

    /// Make a GET request
    async fn get(&self, url: &str) -> Result<RequestResponse>;

    /// Make a request with custom method
    async fn request(&self, method: &str, url: &str) -> Result<RequestResponse> {
        match method.to_uppercase().as_str() {
            "HEAD" => self.head(url).await,
            "GET" => self.get(url).await,
            _ => Err(crate::utils::error::SleuthError::Unknown(format!(
                "Unsupported HTTP method: {}",
                method
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_response_new() {
        let response = RequestResponse::new(200);
        assert_eq!(response.status_code, 200);
        assert!(response.body.is_none());
        assert!(response.headers.is_empty());
    }

    #[test]
    fn test_request_response_with_body() {
        let response = RequestResponse::with_body(200, "test body".to_string());
        assert_eq!(response.status_code, 200);
        assert_eq!(response.body, Some("test body".to_string()));
        assert!(response.headers.is_empty());
    }

    #[test]
    fn test_request_response_different_status_codes() {
        let response_404 = RequestResponse::new(404);
        assert_eq!(response_404.status_code, 404);

        let response_500 = RequestResponse::new(500);
        assert_eq!(response_500.status_code, 500);

        let response_301 = RequestResponse::new(301);
        assert_eq!(response_301.status_code, 301);
    }

    #[test]
    fn test_request_response_clone() {
        let response = RequestResponse::with_body(200, "test".to_string());
        let cloned = response.clone();
        assert_eq!(cloned.status_code, response.status_code);
        assert_eq!(cloned.body, response.body);
    }
}
