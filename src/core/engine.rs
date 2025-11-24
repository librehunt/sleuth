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
}
