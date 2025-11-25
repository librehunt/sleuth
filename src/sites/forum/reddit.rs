use crate::data::site_info::SiteType;
use crate::sites::Site;

/// Reddit username checker
pub struct RedditChecker;

impl RedditChecker {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RedditChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl Site for RedditChecker {
    fn name(&self) -> &str {
        "Reddit"
    }

    fn url_pattern(&self) -> &str {
        "https://www.reddit.com/user/{}"
    }

    fn site_type(&self) -> SiteType {
        SiteType::Forum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reddit_checker_new() {
        let checker = RedditChecker::new();
        assert_eq!(checker.name(), "Reddit");
    }

    #[test]
    fn test_reddit_checker_url_pattern() {
        let checker = RedditChecker::new();
        assert_eq!(checker.url_pattern(), "https://www.reddit.com/user/{}");
    }

    #[test]
    fn test_reddit_checker_build_url() {
        let checker = RedditChecker::new();
        assert_eq!(
            checker.build_url("spez"),
            "https://www.reddit.com/user/spez"
        );
    }

    #[test]
    fn test_reddit_checker_site_type() {
        let checker = RedditChecker::new();
        assert_eq!(checker.site_type(), SiteType::Forum);
    }

    #[test]
    fn test_reddit_checker_http_method() {
        let checker = RedditChecker::new();
        assert_eq!(checker.http_method(), "HEAD");
    }

    #[test]
    fn test_reddit_checker_parse_response() {
        let checker = RedditChecker::new();
        assert_eq!(checker.parse_response(200, None), Some(true));
        assert_eq!(checker.parse_response(404, None), Some(false));
        assert_eq!(checker.parse_response(500, None), None);
    }
}
