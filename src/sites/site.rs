//! Site trait for username checking

use crate::data::site_info::SiteType;

/// Trait that all site checkers must implement
/// Sites are responsible for URL construction and response interpretation,
/// but NOT for making HTTP requests (handled by Scanner/Engine)
pub trait Site: Send + Sync {
    /// Name of the site
    fn name(&self) -> &str;

    /// Get the base URL pattern for this site
    fn url_pattern(&self) -> &str;

    /// Get the site type/category
    fn site_type(&self) -> SiteType;

    /// Build the full URL to check for a username
    fn build_url(&self, username: &str) -> String {
        self.url_pattern().replace("{}", username)
    }

    /// Determine if username exists based on HTTP response
    /// Returns Some(true) if exists, Some(false) if not found, None if uncertain
    fn parse_response(&self, status_code: u16, _body: Option<&str>) -> Option<bool> {
        // Default implementation: 200 = exists, 404 = not found
        match status_code {
            200..=299 => Some(true),
            404 => Some(false),
            _ => None, // Uncertain, might need retry
        }
    }

    /// Get HTTP method to use (default: HEAD for efficiency)
    fn http_method(&self) -> &'static str {
        "HEAD"
    }

    /// Get custom headers if needed (default: none)
    fn headers(&self) -> Vec<(&'static str, &'static str)> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::site_info::SiteType;

    struct TestSite;

    impl Site for TestSite {
        fn name(&self) -> &str {
            "TestSite"
        }

        fn url_pattern(&self) -> &str {
            "https://test.com/{}"
        }

        fn site_type(&self) -> SiteType {
            SiteType::Other
        }
    }

    #[test]
    fn test_site_build_url() {
        let site = TestSite;
        assert_eq!(site.build_url("user"), "https://test.com/user");
    }

    #[test]
    fn test_site_parse_response() {
        let site = TestSite;
        assert_eq!(site.parse_response(200, None), Some(true));
        assert_eq!(site.parse_response(404, None), Some(false));
        assert_eq!(site.parse_response(500, None), None);
    }

    #[test]
    fn test_site_http_method() {
        let site = TestSite;
        assert_eq!(site.http_method(), "HEAD");
    }
}
