//! Search result types

use serde::{Deserialize, Serialize};

/// Result of a username search on a site
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchResult {
    /// Site name where the search was performed
    pub site: String,
    /// Username that was searched
    pub username: String,
    /// Whether the username exists on this site
    pub exists: bool,
    /// URL where the profile was found (if exists)
    pub url: Option<String>,
    /// Additional metadata
    pub metadata: Option<String>,
}

impl SearchResult {
    /// Create a new search result
    pub fn new(site: String, username: String, exists: bool) -> Self {
        Self {
            site,
            username,
            exists,
            url: None,
            metadata: None,
        }
    }

    /// Create a result indicating the username exists
    pub fn found(site: String, username: String, url: String) -> Self {
        Self {
            site,
            username,
            exists: true,
            url: Some(url),
            metadata: None,
        }
    }

    /// Create a result indicating the username was not found
    pub fn not_found(site: String, username: String) -> Self {
        Self {
            site,
            username,
            exists: false,
            url: None,
            metadata: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_result_new() {
        let result = SearchResult::new("github".to_string(), "testuser".to_string(), true);
        assert_eq!(result.site, "github");
        assert_eq!(result.username, "testuser");
        assert!(result.exists);
        assert!(result.url.is_none());
    }

    #[test]
    fn test_search_result_found() {
        let result = SearchResult::found(
            "github".to_string(),
            "testuser".to_string(),
            "https://github.com/testuser".to_string(),
        );
        assert!(result.exists);
        assert_eq!(result.url, Some("https://github.com/testuser".to_string()));
    }

    #[test]
    fn test_search_result_not_found() {
        let result = SearchResult::not_found("github".to_string(), "testuser".to_string());
        assert!(!result.exists);
        assert!(result.url.is_none());
    }
}
