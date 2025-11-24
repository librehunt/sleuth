//! Site trait for username checking

use crate::core::result::SearchResult;
use crate::data::site_info::SiteType;
use crate::utils::error::Result;
use async_trait::async_trait;

/// Trait that all site checkers must implement
#[async_trait]
pub trait Site: Send + Sync {
    /// Name of the site
    fn name(&self) -> &str;

    /// Check if a username exists on this site
    async fn check_username(&self, username: &str) -> Result<SearchResult>;

    /// Get the base URL pattern for this site
    fn url_pattern(&self) -> &str;

    /// Get the site type/category
    fn site_type(&self) -> SiteType;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_site_trait_definition() {
        // Trait definition test
        assert!(true);
    }
}
