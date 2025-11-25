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
    fn parse_response(&self, _username: &str, status_code: u16, _body: Option<&str>) -> Option<bool> {
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

    /// Whether this site requires browser rendering (JavaScript execution)
    /// Sites that render content dynamically should return true
    fn requires_browser(&self) -> bool {
        false // Default: no browser needed
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
        assert_eq!(site.parse_response("testuser", 200, None), Some(true));
        assert_eq!(site.parse_response("testuser", 404, None), Some(false));
        assert_eq!(site.parse_response("testuser", 500, None), None);
    }

    #[test]
    fn test_site_http_method() {
        let site = TestSite;
        assert_eq!(site.http_method(), "HEAD");
    }

    #[test]
    fn test_site_headers() {
        let site = TestSite;
        let headers = site.headers();
        assert!(headers.is_empty());
    }

    #[test]
    fn test_site_build_url_with_special_chars() {
        let site = TestSite;
        let url = site.build_url("user-name_123");
        assert_eq!(url, "https://test.com/user-name_123");
    }

    #[test]
    fn test_site_parse_response_edge_cases() {
        let site = TestSite;
        // Test various status codes
        assert_eq!(site.parse_response("testuser", 200, None), Some(true));
        assert_eq!(site.parse_response("testuser", 201, None), Some(true));
        assert_eq!(site.parse_response("testuser", 299, None), Some(true));
        assert_eq!(site.parse_response("testuser", 404, None), Some(false));
        assert_eq!(site.parse_response("testuser", 500, None), None);
        assert_eq!(site.parse_response("testuser", 301, None), None);
        assert_eq!(site.parse_response("testuser", 302, None), None);
    }

    #[test]
    fn test_site_parse_response_with_body() {
        let site = TestSite;
        // Body is currently ignored in default implementation
        assert_eq!(site.parse_response("testuser", 200, Some("body")), Some(true));
        assert_eq!(site.parse_response("testuser", 404, Some("not found")), Some(false));
    }

    // Test custom site implementation
    struct CustomSite {
        method: &'static str,
    }

    impl Site for CustomSite {
        fn name(&self) -> &str {
            "CustomSite"
        }

        fn url_pattern(&self) -> &str {
            "https://custom.com/{}"
        }

        fn site_type(&self) -> SiteType {
            SiteType::Other
        }

        fn http_method(&self) -> &'static str {
            self.method
        }

        fn parse_response(&self, _username: &str, status_code: u16, _body: Option<&str>) -> Option<bool> {
            // Custom logic: only 200 is true, everything else is false
            if status_code == 200 {
                Some(true)
            } else {
                Some(false)
            }
        }
    }

    #[test]
    fn test_custom_site_implementation() {
        let site = CustomSite { method: "GET" };
        assert_eq!(site.name(), "CustomSite");
        assert_eq!(site.http_method(), "GET");
        assert_eq!(site.parse_response("testuser", 200, None), Some(true));
        assert_eq!(site.parse_response("testuser", 404, None), Some(false));
        assert_eq!(site.parse_response("testuser", 500, None), Some(false));
    }

    #[test]
    fn test_custom_site_headers() {
        struct SiteWithHeaders;

        impl Site for SiteWithHeaders {
            fn name(&self) -> &str {
                "SiteWithHeaders"
            }

            fn url_pattern(&self) -> &str {
                ""
            }

            fn site_type(&self) -> SiteType {
                SiteType::Other
            }

            fn headers(&self) -> Vec<(&'static str, &'static str)> {
                vec![("Authorization", "Bearer token"), ("X-Custom", "value")]
            }
        }

        let site = SiteWithHeaders;
        let headers = site.headers();
        assert_eq!(headers.len(), 2);
        assert_eq!(headers[0], ("Authorization", "Bearer token"));
        assert_eq!(headers[1], ("X-Custom", "value"));
    }
}
