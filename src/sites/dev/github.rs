//! GitHub site checker

use crate::core::result::SearchResult;
use crate::data::site_info::SiteType;
use crate::sites::Site;
use crate::utils::error::Result;
use async_trait::async_trait;

/// GitHub site checker
pub struct GitHubChecker {
    // Fields can be added here as needed (e.g., HTTP client)
}

impl GitHubChecker {
    /// Create a new GitHub checker
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for GitHubChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Site for GitHubChecker {
    fn name(&self) -> &str {
        "GitHub"
    }

    fn site_type(&self) -> SiteType {
        SiteType::Dev
    }

    fn url_pattern(&self) -> &str {
        "https://github.com/{}"
    }

    async fn check_username(&self, username: &str) -> Result<SearchResult> {
        // TODO: Implement actual GitHub username checking
        // For now, return a placeholder
        Ok(SearchResult::not_found(
            self.name().to_string(),
            username.to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_github_checker_name() {
        let checker = GitHubChecker::new();
        assert_eq!(checker.name(), "GitHub");
    }

    #[test]
    fn test_github_checker_site_type() {
        let checker = GitHubChecker::new();
        assert_eq!(checker.site_type(), SiteType::Dev);
    }

    #[test]
    fn test_github_checker_url_pattern() {
        let checker = GitHubChecker::new();
        assert_eq!(checker.url_pattern(), "https://github.com/{}");
    }

    #[tokio::test]
    async fn test_github_checker_check_username() {
        let checker = GitHubChecker::new();
        let result = checker.check_username("testuser").await;
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.site, "GitHub");
        assert_eq!(result.username, "testuser");
    }
}
