use crate::data::site_info::SiteType;
use crate::sites::Site;

/// OnlyFans username checker
pub struct OnlyFansChecker;

impl OnlyFansChecker {
    pub fn new() -> Self {
        Self
    }
}

impl Default for OnlyFansChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl Site for OnlyFansChecker {
    fn name(&self) -> &str {
        "OnlyFans"
    }

    fn url_pattern(&self) -> &str {
        "https://onlyfans.com/{}"
    }

    fn site_type(&self) -> SiteType {
        SiteType::Nsfw
    }

    fn requires_browser(&self) -> bool {
        true // OnlyFans renders content with JavaScript
    }

    fn parse_response(&self, username: &str, status_code: u16, body: Option<&str>) -> Option<bool> {
        match status_code {
            404 => Some(false),
            200..=299 => {
                if let Some(body_text) = body {
                    let body_lc = body_text.to_ascii_lowercase();
                    
                    // Check for error page structure (negative indicators)
                    // Error pages have specific CSS classes like "b-404", "b-wrapper-404"
                    let has_error_structure = 
                        body_lc.contains("b-404")
                        || body_lc.contains("b-wrapper-404")
                        || body_lc.contains("b-404__title")
                        || body_lc.contains("b-404__description");
                    
                    if has_error_structure {
                        return Some(false);
                    }
                    
                    // Check for valid profile structure (positive indicators)
                    // Valid profiles have specific CSS classes and IDs that indicate a real profile page
                    let has_profile_structure = 
                        // Profile-specific CSS classes
                        body_lc.contains("b-profile")
                        || body_lc.contains("b-profile__header")
                        || body_lc.contains("b-profile__user")
                        || body_lc.contains("b-profile__content")
                        || body_lc.contains("b-profile-info-card")
                        || body_lc.contains("b-username")
                        || body_lc.contains("b-username-row")
                        // Avatar/image classes
                        || body_lc.contains("g-avatar")
                        || body_lc.contains("b-friend__avatar")
                        // Profile-specific IDs
                        || body_lc.contains("profileposttab")
                        || body_lc.contains("icon-profile")
                        || body_lc.contains("icon-media")
                        || body_lc.contains("icon-post");
                    
                    // If we detect profile structure, it's a valid profile
                    if has_profile_structure {
                        return Some(true);
                    }
                    
                    // If body is very short, likely an error page
                    if body_text.len() < 3000 {
                        return Some(false);
                    }
                    
                    // If content is substantial (> 20KB) and no error structure, assume valid
                    // (large pages are usually valid profiles with lots of content)
                    if body_text.len() > 20000 {
                        return Some(true);
                    }
                    
                    // Default to false to avoid false positives
                    // If we can't find clear positive indicators, assume not found
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
        // Need body to detect false positives
        "GET"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_onlyfans_checker_new() {
        let checker = OnlyFansChecker::new();
        assert_eq!(checker.name(), "OnlyFans");
    }

    #[test]
    fn test_onlyfans_checker_http_method() {
        let checker = OnlyFansChecker::new();
        assert_eq!(checker.http_method(), "GET");
    }

    #[test]
    fn test_onlyfans_checker_url_pattern() {
        let checker = OnlyFansChecker::new();
        assert_eq!(checker.url_pattern(), "https://onlyfans.com/{}");
    }

    #[test]
    fn test_onlyfans_checker_build_url() {
        let checker = OnlyFansChecker::new();
        assert_eq!(
            checker.build_url("testuser"),
            "https://onlyfans.com/testuser"
        );
    }

    #[test]
    fn test_onlyfans_checker_site_type() {
        let checker = OnlyFansChecker::new();
        assert_eq!(checker.site_type(), SiteType::Nsfw);
    }

    #[test]
    fn test_onlyfans_checker_false_positive_page_not_found() {
        let checker = OnlyFansChecker::new();
        let body = r#"<html><body>PAGE NOT FOUND</body></html>"#;
        assert_eq!(checker.parse_response("testuser", 200, Some(body)), Some(false));
    }

    #[test]
    fn test_onlyfans_checker_false_positive_not_found_text() {
        let checker = OnlyFansChecker::new();
        let body = r#"<html><body>Not Found</body></html>"#;
        assert_eq!(checker.parse_response("testuser", 200, Some(body)), Some(false));
    }

    #[test]
    fn test_onlyfans_checker_parse_response() {
        let checker = OnlyFansChecker::new();
        assert_eq!(checker.parse_response("testuser", 200, None), Some(true));
        assert_eq!(checker.parse_response("testuser", 404, None), Some(false));
        assert_eq!(checker.parse_response("testuser", 500, None), None);
    }
}
