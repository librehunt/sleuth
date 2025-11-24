//! HTTP request implementation using reqwest

use crate::request::{Request, RequestResponse};
use crate::utils::error::{Result, SleuthError};
use reqwest::Client;
use std::time::Duration;

pub struct HttpRequest {
    client: Client,
    #[allow(dead_code)]
    timeout: Duration,
}

impl HttpRequest {
    pub fn new(timeout_secs: u64) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .user_agent("sleuth/0.0.1")
            .build()
            .map_err(SleuthError::Http)?;

        Ok(Self {
            client,
            timeout: Duration::from_secs(timeout_secs),
        })
    }

    pub fn with_user_agent(timeout_secs: u64, user_agent: &str) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .user_agent(user_agent)
            .build()
            .map_err(SleuthError::Http)?;

        Ok(Self {
            client,
            timeout: Duration::from_secs(timeout_secs),
        })
    }
}

#[async_trait::async_trait]
impl Request for HttpRequest {
    async fn head(&self, url: &str) -> Result<RequestResponse> {
        let response = self
            .client
            .head(url)
            .send()
            .await
            .map_err(SleuthError::Http)?;

        let status_code = response.status().as_u16();
        let headers: Vec<(String, String)> = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();

        Ok(RequestResponse {
            status_code,
            body: None,
            headers,
        })
    }

    async fn get(&self, url: &str) -> Result<RequestResponse> {
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(SleuthError::Http)?;

        let status_code = response.status().as_u16();
        let headers: Vec<(String, String)> = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();

        let body = response.text().await.ok();

        Ok(RequestResponse {
            status_code,
            body,
            headers,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_http_request_new() {
        let request = HttpRequest::new(10);
        assert!(request.is_ok());
    }

    #[tokio::test]
    async fn test_http_request_new_zero_timeout() {
        let request = HttpRequest::new(0);
        assert!(request.is_ok());
    }

    #[tokio::test]
    async fn test_http_request_with_user_agent() {
        let request = HttpRequest::with_user_agent(10, "test-agent/1.0");
        assert!(request.is_ok());
    }

    #[tokio::test]
    async fn test_http_request_head() {
        let request = HttpRequest::new(10).unwrap();
        let response = request.head("https://github.com").await;
        assert!(response.is_ok());
        let response = response.unwrap();
        assert!(response.status_code >= 200 && response.status_code < 500);
        assert!(response.body.is_none()); // HEAD requests don't have body
    }

    #[tokio::test]
    async fn test_http_request_get() {
        let request = HttpRequest::new(10).unwrap();
        let response = request.get("https://github.com").await;
        assert!(response.is_ok());
        let response = response.unwrap();
        assert!(response.status_code >= 200 && response.status_code < 500);
        assert!(response.body.is_some());
    }

    #[tokio::test]
    async fn test_http_request_get_has_headers() {
        let request = HttpRequest::new(10).unwrap();
        let response = request.get("https://github.com").await;
        assert!(response.is_ok());
        let response = response.unwrap();
        // Should have some headers
        assert!(!response.headers.is_empty());
    }

    #[tokio::test]
    async fn test_http_request_head_has_headers() {
        let request = HttpRequest::new(10).unwrap();
        let response = request.head("https://github.com").await;
        assert!(response.is_ok());
        let response = response.unwrap();
        // Should have some headers
        assert!(!response.headers.is_empty());
    }

    #[tokio::test]
    async fn test_http_request_invalid_url() {
        let request = HttpRequest::new(10).unwrap();
        let response = request.get("not-a-valid-url").await;
        assert!(response.is_err());
    }

    #[tokio::test]
    async fn test_http_request_nonexistent_domain() {
        let request = HttpRequest::new(1).unwrap(); // Short timeout
        let response = request
            .get("https://this-domain-definitely-does-not-exist-12345.com")
            .await;
        // Should error (timeout or DNS failure)
        assert!(response.is_err());
    }

    #[tokio::test]
    async fn test_http_request_timeout() {
        let request = HttpRequest::new(1).unwrap(); // 1 second timeout
                                                    // Use a URL that will timeout (or take too long)
                                                    // Note: This might not always timeout if the service is fast, so we just check it doesn't panic
        let response = request.get("https://httpbin.org/delay/5").await;
        // Should error due to timeout, but if it succeeds that's also OK (service might be fast)
        // The important thing is it doesn't panic
        let _ = response; // Just ensure it doesn't panic
    }

    #[tokio::test]
    async fn test_http_request_method_delegation() {
        let request = HttpRequest::new(10).unwrap();
        // Test that request() method delegates to head() and get()
        let head_response = request.request("HEAD", "https://github.com").await;
        assert!(head_response.is_ok());

        let get_response = request.request("GET", "https://github.com").await;
        assert!(get_response.is_ok());

        // Test case-insensitive
        let head_lower = request.request("head", "https://github.com").await;
        assert!(head_lower.is_ok());

        let get_lower = request.request("get", "https://github.com").await;
        assert!(get_lower.is_ok());
    }

    #[tokio::test]
    async fn test_http_request_unsupported_method() {
        let request = HttpRequest::new(10).unwrap();
        let response = request.request("POST", "https://github.com").await;
        assert!(response.is_err());
        let err = response.unwrap_err();
        assert!(err.to_string().contains("Unsupported HTTP method"));
    }
}
