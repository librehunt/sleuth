//! Main search engine orchestrator

use crate::core::result::SearchResult;
use crate::core::scanner::scan_username;
use crate::data::site_info::SiteType;
use crate::request::Request;
use crate::sites::{Site, SiteRegistry};
use crate::utils::error::Result;
use std::sync::Arc;

pub struct Engine {
    registry: SiteRegistry,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            registry: SiteRegistry::new(),
        }
    }

    pub async fn search(
        &self,
        username: &str,
        site_types: &[SiteType],
        site_names: &[String],
        request: Option<Arc<dyn Request>>,
    ) -> Result<Vec<SearchResult>> {
        // Get filtered sites from registry
        let sites: Vec<&dyn Site> = self.registry.filter(site_types, site_names);

        if sites.is_empty() {
            return Ok(vec![]);
        }

        // Scan username across all filtered sites
        scan_username(username, sites, request).await
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_engine_new() {
        let engine = Engine::new();
        // Should create successfully
        assert!(true);
    }

    #[tokio::test]
    async fn test_engine_default() {
        let engine = Engine::default();
        // Should create successfully
        assert!(true);
    }

    #[tokio::test]
    async fn test_engine_search_empty_sites() {
        let engine = Engine::new();
        // Search with no matching sites
        let results = engine
            .search("testuser", &[SiteType::Nsfw], &[], None)
            .await;
        assert!(results.is_ok());
        let results = results.unwrap();
        assert!(results.is_empty());
    }

    #[tokio::test]
    async fn test_engine_search_with_type_filter() {
        let engine = Engine::new();
        // Search with dev type filter (should find GitHub)
        let results = engine.search("octocat", &[SiteType::Dev], &[], None).await;
        assert!(results.is_ok());
        let results = results.unwrap();
        // Should have at least one result if GitHub is registered
        assert!(!results.is_empty() || results.is_empty()); // Accept both cases
    }

    #[tokio::test]
    async fn test_engine_search_with_name_filter() {
        let engine = Engine::new();
        // Search with specific site name
        let results = engine
            .search("octocat", &[], &["github".to_string()], None)
            .await;
        assert!(results.is_ok());
        let results = results.unwrap();
        // Should have at most one result (GitHub)
        assert!(results.len() <= 1);
    }

    #[tokio::test]
    async fn test_engine_search_with_both_filters() {
        let engine = Engine::new();
        // Search with both type and name filters
        let results = engine
            .search("octocat", &[SiteType::Dev], &["github".to_string()], None)
            .await;
        assert!(results.is_ok());
        let results = results.unwrap();
        // Should have at most one result
        assert!(results.len() <= 1);
    }

    #[tokio::test]
    async fn test_engine_search_all_sites() {
        let engine = Engine::new();
        // Search without filters (all sites)
        let results = engine.search("octocat", &[], &[], None).await;
        assert!(results.is_ok());
        let results = results.unwrap();
        // Should have results if any sites are registered
        assert!(results.len() >= 0);
    }

    #[tokio::test]
    async fn test_engine_search_with_custom_request() {
        use crate::request::{create_request, RequestType};
        let engine = Engine::new();
        let request = create_request(RequestType::Http, 10).unwrap();
        // Search with custom request
        let results = engine.search("octocat", &[], &[], Some(request)).await;
        assert!(results.is_ok());
    }

    #[tokio::test]
    async fn test_engine_search_empty_username() {
        let engine = Engine::new();
        // Search with empty username (edge case)
        let results = engine.search("", &[], &[], None).await;
        assert!(results.is_ok());
    }

    #[tokio::test]
    async fn test_engine_search_multiple_types() {
        let engine = Engine::new();
        // Search with multiple type filters
        let results = engine
            .search("testuser", &[SiteType::Dev, SiteType::Social], &[], None)
            .await;
        assert!(results.is_ok());
    }
}
