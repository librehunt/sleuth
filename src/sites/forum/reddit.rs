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

    fn requires_browser(&self) -> bool {
        true // Reddit renders content with JavaScript
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
                if let Some(body_text) = body {
                    let body_lc = body_text.to_ascii_lowercase();

                    // Check for explicit error messages first (negative indicators)
                    if body_lc.contains("sorry, there isn") || body_lc.contains("page not found") {
                        return Some(false);
                    }

                    // Check for positive indicators: Reddit profile pages typically contain:
                    // - Profile-specific elements like "shreddit-user-profile" or similar
                    // - User activity indicators
                    // - Profile metadata
                    // Blocked/error pages typically don't have these

                    // Check for profile-specific content that indicates a valid profile
                    // Reddit uses specific data attributes and classes for user profiles
                    let has_profile_indicators = body_lc.contains("shreddit-user-profile")
                        || body_lc.contains("user-profile")
                        || body_lc.contains("profile-overview")
                        || body_lc.contains("user-activity")
                        || (body_lc.contains("karma") && body_lc.contains("cake-day"))
                        || body_lc.contains("reddit-profile");

                    // Also check if the page has substantial content beyond just CSS/JS
                    // Blocked pages are often mostly CSS/JS with minimal actual content
                    let has_substantial_content = body_text.len() > 50000; // Valid profiles are usually > 50KB

                    if has_profile_indicators && has_substantial_content {
                        return Some(true);
                    }

                    // If body is very short, likely an error page
                    if body_text.len() < 10000 {
                        return Some(false);
                    }

                    // Default to false to avoid false positives
                    // If we can't find clear positive indicators, assume it's not a valid profile
                    Some(false)
                } else {
                    // No body - can't determine
                    None
                }
            }
            _ => None,
        }
    }

    fn http_method(&self) -> &'static str {
        "GET"
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
        assert_eq!(checker.http_method(), "GET");
    }

    #[test]
    fn test_reddit_checker_false_positive_sorry_message() {
        let checker = RedditChecker::new();
        let body = r#"<html><body>Sorry, there isn't anything here</body></html>"#;
        assert_eq!(
            checker.parse_response("testuser", 200, Some(body)),
            Some(false)
        );
    }

    #[test]
    fn test_reddit_checker_false_positive_page_not_found() {
        let checker = RedditChecker::new();
        let body = r#"<html><body>page not found</body></html>"#;
        assert_eq!(
            checker.parse_response("testuser", 200, Some(body)),
            Some(false)
        );
    }

    #[test]
    fn test_reddit_checker_false_positive_blocked() {
        let checker = RedditChecker::new();
        let body = r#"<html><body>You've been blocked by network security</body></html>"#;
        assert_eq!(
            checker.parse_response("testuser", 200, Some(body)),
            Some(false)
        );
    }

    #[test]
    fn test_reddit_checker_parse_response() {
        let checker = RedditChecker::new();
        // With body=None (HEAD request), we can't determine - returns None
        assert_eq!(checker.parse_response("testuser", 200, None), None);
        assert_eq!(checker.parse_response("testuser", 404, None), Some(false));
        assert_eq!(checker.parse_response("testuser", 500, None), None);

        // With body containing profile structure, should return true
        // Need substantial content (> 50KB) and profile indicators
        let mut body_with_profile = String::from(
            r#"<html><body class="shreddit-user-profile"><div>karma</div><div>cake-day</div>"#,
        );
        body_with_profile.push_str(&"x".repeat(60000)); // Make it > 50KB
        body_with_profile.push_str("</body></html>");
        assert_eq!(
            checker.parse_response("testuser", 200, Some(&body_with_profile)),
            Some(true)
        );
    }
}
