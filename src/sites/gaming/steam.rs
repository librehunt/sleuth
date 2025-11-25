use crate::data::site_info::SiteType;
use crate::sites::Site;

/// Steam username checker
pub struct SteamChecker;

impl SteamChecker {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SteamChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl Site for SteamChecker {
    fn name(&self) -> &str {
        "Steam"
    }

    fn url_pattern(&self) -> &str {
        "https://steamcommunity.com/id/{}"
    }

    fn site_type(&self) -> SiteType {
        SiteType::Gaming
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

                    // Check for error page structure (negative indicators)
                    // Error pages have specific indicators that don't depend on language
                    let has_error_structure = body_lc.contains("steam community :: error")
                        || body_lc.contains("fatalerror.css")
                        || body_lc.contains("error_ctn")
                        || body_lc.contains("class=\"error");

                    if has_error_structure {
                        return Some(false);
                    }

                    // Check for valid profile structure (positive indicators)
                    // Valid profiles have specific CSS classes and structure
                    let has_profile_structure =
                        // Profile page class (most reliable indicator)
                        body_lc.contains("profile_page")
                        // Profile-specific elements
                        || (body_lc.contains("persona_name") && body_lc.contains("profile_content"))
                        || body_lc.contains("profile_header")
                        || body_lc.contains("playeravatar")
                        || body_lc.contains("profile_badges")
                        || body_lc.contains("profile_summary");

                    // If we detect profile structure, it's a valid profile
                    if has_profile_structure {
                        return Some(true);
                    }

                    // If body is very short, likely an error page
                    if body_text.len() < 5000 {
                        return Some(false);
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
        // Use GET instead of HEAD to get response body for parsing
        "GET"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_steam_checker_new() {
        let checker = SteamChecker::new();
        assert_eq!(checker.name(), "Steam");
    }

    #[test]
    fn test_steam_checker_url_pattern() {
        let checker = SteamChecker::new();
        assert_eq!(checker.url_pattern(), "https://steamcommunity.com/id/{}");
    }

    #[test]
    fn test_steam_checker_build_url() {
        let checker = SteamChecker::new();
        assert_eq!(
            checker.build_url("gaben"),
            "https://steamcommunity.com/id/gaben"
        );
    }

    #[test]
    fn test_steam_checker_site_type() {
        let checker = SteamChecker::new();
        assert_eq!(checker.site_type(), SiteType::Gaming);
    }

    #[test]
    fn test_steam_checker_http_method() {
        let checker = SteamChecker::new();
        assert_eq!(checker.http_method(), "GET");
    }

    #[test]
    fn test_steam_checker_parse_response() {
        let checker = SteamChecker::new();
        // With body=None (HEAD request), we can't determine - returns None
        assert_eq!(checker.parse_response("testuser", 200, None), None);
        // 404 status
        assert_eq!(checker.parse_response("testuser", 404, None), Some(false));
        // 500 status
        assert_eq!(checker.parse_response("testuser", 500, None), None);

        // Valid profile with profile_page class
        let body_with_profile = r#"<html><body class="flat_page profile_page"><div class="persona_name">testuser</div></body></html>"#;
        assert_eq!(
            checker.parse_response("testuser", 200, Some(body_with_profile)),
            Some(true)
        );
    }

    #[test]
    fn test_steam_checker_false_positive_error_page() {
        let checker = SteamChecker::new();
        // Error page with error_ctn class
        let body_error = r#"<html><title>Steam Community :: Error</title><body><div class="error_ctn">Error</div></body></html>"#;
        assert_eq!(
            checker.parse_response("testuser", 200, Some(body_error)),
            Some(false)
        );
    }

    #[test]
    fn test_steam_checker_false_positive_fatalerror() {
        let checker = SteamChecker::new();
        // Error page with fatalerror.css
        let body_error = r#"<html><link href="fatalerror.css"><body>Error</body></html>"#;
        assert_eq!(
            checker.parse_response("testuser", 200, Some(body_error)),
            Some(false)
        );
    }
}
