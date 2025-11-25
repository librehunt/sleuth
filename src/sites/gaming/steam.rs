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

    fn parse_response(&self, _username: &str, status_code: u16, body: Option<&str>) -> Option<bool> {
        match status_code {
            404 => Some(false),
            200..=299 => {
                // Steam returns 200 even for non-existent profiles
                // Check response body for error indicators
                if let Some(body_text) = body {
                    // French error message
                    if body_text.contains("Profil spécifié introuvable") {
                        return Some(false);
                    }
                    // English error message
                    if body_text.contains("The specified profile could not be found") {
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
        // Valid profile (200 without error message)
        assert_eq!(
            checker.parse_response("testuser", 200, Some("<html>Profile content</html>")),
            Some(true)
        );
        // 404 status
        assert_eq!(checker.parse_response("testuser", 404, None), Some(false));
        // 500 status
        assert_eq!(checker.parse_response("testuser", 500, None), None);
    }

    #[test]
    fn test_steam_checker_false_positive_french() {
        let checker = SteamChecker::new();
        let body = r#"<html><body>Profil spécifié introuvable</body></html>"#;
        assert_eq!(checker.parse_response("testuser", 200, Some(body)), Some(false));
    }

    #[test]
    fn test_steam_checker_false_positive_english() {
        let checker = SteamChecker::new();
        let body = r#"<html><body>The specified profile could not be found</body></html>"#;
        assert_eq!(checker.parse_response("testuser", 200, Some(body)), Some(false));
    }
}
