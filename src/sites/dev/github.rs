//! GitHub site checker

use crate::data::site_info::SiteType;
use crate::sites::Site;

/// GitHub site checker
pub struct GitHubChecker;

impl GitHubChecker {
    /// Create a new GitHub checker
    pub fn new() -> Self {
        Self
    }
}

impl Default for GitHubChecker {
    fn default() -> Self {
        Self::new()
    }
}

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

    // Uses default build_url() implementation
    // Uses default parse_response() implementation (200 = exists, 404 = not found)
    // Uses default http_method() (HEAD)
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

    #[test]
    fn test_github_checker_build_url() {
        let checker = GitHubChecker::new();
        assert_eq!(checker.build_url("testuser"), "https://github.com/testuser");
    }

    #[test]
    fn test_github_checker_parse_response() {
        let checker = GitHubChecker::new();
        assert_eq!(checker.parse_response("testuser", 200, None), Some(true));
        assert_eq!(checker.parse_response("testuser", 404, None), Some(false));
    }
}
