use crate::data::site_info::SiteType;
use crate::sites::Site;

/// Twitter/X username checker
pub struct TwitterChecker;

impl TwitterChecker {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TwitterChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl Site for TwitterChecker {
    fn name(&self) -> &str {
        "Twitter"
    }

    fn url_pattern(&self) -> &str {
        "https://x.com/{}"
    }

    fn site_type(&self) -> SiteType {
        SiteType::Social
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_twitter_checker_new() {
        let checker = TwitterChecker::new();
        assert_eq!(checker.name(), "Twitter");
    }

    #[test]
    fn test_twitter_checker_url_pattern() {
        let checker = TwitterChecker::new();
        assert_eq!(checker.url_pattern(), "https://x.com/{}");
    }

    #[test]
    fn test_twitter_checker_build_url() {
        let checker = TwitterChecker::new();
        assert_eq!(checker.build_url("elonmusk"), "https://x.com/elonmusk");
    }

    #[test]
    fn test_twitter_checker_site_type() {
        let checker = TwitterChecker::new();
        assert_eq!(checker.site_type(), SiteType::Social);
    }

    #[test]
    fn test_twitter_checker_http_method() {
        let checker = TwitterChecker::new();
        assert_eq!(checker.http_method(), "HEAD");
    }

    #[test]
    fn test_twitter_checker_parse_response() {
        let checker = TwitterChecker::new();
        assert_eq!(checker.parse_response(200, None), Some(true));
        assert_eq!(checker.parse_response(404, None), Some(false));
        assert_eq!(checker.parse_response(500, None), None);
    }
}
