//! Browser request implementation using headless Chrome

use crate::request::{Request, RequestResponse};
use crate::utils::error::{Result, SleuthError};
use headless_chrome::Browser;
use std::time::Duration;

pub struct BrowserRequest {
    timeout: Duration,
}

impl BrowserRequest {
    pub fn new(timeout_secs: u64) -> Result<Self> {
        Ok(Self {
            timeout: Duration::from_secs(timeout_secs),
        })
    }

    fn launch_browser(&self) -> Result<Browser> {
        Browser::default().map_err(|e| {
            SleuthError::Unknown(format!("Failed to launch browser: {}", e))
        })
    }
}

#[async_trait::async_trait]
impl Request for BrowserRequest {
    async fn head(&self, url: &str) -> Result<RequestResponse> {
        // For HEAD requests, we still need to load the page to get headers
        // But we can optimize by not waiting for full render
        self.get(url).await
    }

    async fn get(&self, url: &str) -> Result<RequestResponse> {
        // Launch browser in a blocking way (headless_chrome is not async)
        // We'll use tokio::task::spawn_blocking to run it in a thread pool
        let url = url.to_string();
        let timeout = self.timeout;

        tokio::task::spawn_blocking(move || {
            let browser = BrowserRequest::new(timeout.as_secs())?.launch_browser()?;
            let tab = browser.new_tab().map_err(|e| {
                SleuthError::Unknown(format!("Failed to create tab: {}", e))
            })?;

            // Navigate to URL
            tab.navigate_to(&url).map_err(|e| {
                SleuthError::Unknown(format!("Failed to navigate: {}", e))
            })?;

            // Wait for network to be idle (page loaded)
            tab.wait_until_navigated().map_err(|e| {
                SleuthError::Unknown(format!("Failed to wait for navigation: {}", e))
            })?;

            // Wait a bit for JavaScript to render (configurable)
            // For sites like OnlyFans, we need to wait longer for content to load
            std::thread::sleep(Duration::from_millis(2000));

            // Get the rendered HTML using evaluate_expression
            let body = tab.evaluate("document.documentElement.outerHTML", false)
                .map_err(|e| {
                    SleuthError::Unknown(format!("Failed to get content: {}", e))
                })?;
            
            let body_str = body.value
                .and_then(|v| v.as_str().map(|s| s.to_string()))
                .ok_or_else(|| {
                    SleuthError::Unknown("Failed to extract HTML content".to_string())
                })?;

            // Debug: Save HTML to file for analysis (if SLEUTH_DEBUG_HTML env var is set)
            if let Ok(debug_file) = std::env::var("SLEUTH_DEBUG_HTML") {
                if !debug_file.is_empty() {
                    let _ = std::fs::write(&debug_file, &body_str);
                }
            }

            // Get status code (browser doesn't expose this directly, so we'll use 200 as default)
            // In a real browser, we'd check the network response
            let status_code = 200;

            // Get headers (limited in headless_chrome)
            let headers = vec![];

            Ok(RequestResponse {
                status_code,
                body: Some(body_str),
                headers,
            })
        })
        .await
        .map_err(|e| SleuthError::Unknown(format!("Browser task failed: {}", e)))?
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_browser_request_new() {
        let request = BrowserRequest::new(10);
        assert!(request.is_ok());
    }

    #[test]
    fn test_browser_request_new_zero_timeout() {
        let request = BrowserRequest::new(0);
        assert!(request.is_ok());
    }
}

