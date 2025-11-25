use crate::data::site_info::SiteType;
use crate::sites::Site;

/// Medium username checker
pub struct MediumChecker;

impl MediumChecker {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MediumChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl Site for MediumChecker {
    fn name(&self) -> &str {
        "Medium"
    }

    fn url_pattern(&self) -> &str {
        "https://medium.com/@{}"
    }

    fn site_type(&self) -> SiteType {
        SiteType::Other
    }

    fn parse_response(
        &self,
        _username: &str,
        status_code: u16,
        body: Option<&str>,
    ) -> Option<bool> {
        match status_code {
            404 => Some(false),
            200..=299 => {
                // Medium returns 200 even for non-existent profiles
                // Check response body for error indicators
                if let Some(body_text) = body {
                    // Check for "PAGE NOT FOUND" text
                    if body_text.contains("PAGE NOT FOUND") {
                        return Some(false);
                    }
                    // Check for the specific error message
                    if body_text.contains("Out of nothing, something.") {
                        return Some(false);
                    }
                }
                Some(true)
            }
            _ => None,
        }
    }

    fn http_method(&self) -> &'static str {
        // Use GET instead of HEAD to get response body for parsing
        "GET"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_medium_checker_new() {
        let checker = MediumChecker::new();
        assert_eq!(checker.name(), "Medium");
    }

    #[test]
    fn test_medium_checker_url_pattern() {
        let checker = MediumChecker::new();
        assert_eq!(checker.url_pattern(), "https://medium.com/@{}");
    }

    #[test]
    fn test_medium_checker_build_url() {
        let checker = MediumChecker::new();
        assert_eq!(
            checker.build_url("testauthor"),
            "https://medium.com/@testauthor"
        );
    }

    #[test]
    fn test_medium_checker_site_type() {
        let checker = MediumChecker::new();
        assert_eq!(checker.site_type(), SiteType::Other);
    }

    #[test]
    fn test_medium_checker_http_method() {
        let checker = MediumChecker::new();
        assert_eq!(checker.http_method(), "GET");
    }

    #[test]
    fn test_medium_checker_parse_response() {
        let checker = MediumChecker::new();
        // Valid profile (200 without error message)
        assert_eq!(
            checker.parse_response("testauthor", 200, Some("<html>Author content</html>")),
            Some(true)
        );
        // 404 status
        assert_eq!(checker.parse_response("testauthor", 404, None), Some(false));
        // 500 status
        assert_eq!(checker.parse_response("testauthor", 500, None), None);
    }

    #[test]
    fn test_medium_checker_false_positive_page_not_found() {
        let checker = MediumChecker::new();
        let body = r#"<html><body><h1>PAGE NOT FOUND</h1><p>404</p></body></html>"#;
        assert_eq!(
            checker.parse_response("testauthor", 200, Some(body)),
            Some(false)
        );
    }

    #[test]
    fn test_medium_checker_false_positive_error_message() {
        let checker = MediumChecker::new();
        let body = r#"<html><body>Out of nothing, something.</body></html>"#;
        assert_eq!(
            checker.parse_response("testauthor", 200, Some(body)),
            Some(false)
        );
    }
}
