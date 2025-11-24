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
    async fn test_http_request_head() {
        let request = HttpRequest::new(10).unwrap();
        let response = request.head("https://github.com").await;
        assert!(response.is_ok());
        let response = response.unwrap();
        assert!(response.status_code >= 200 && response.status_code < 500);
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
}
