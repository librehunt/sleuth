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
        assert_eq!(checker.http_method(), "HEAD");
    }

    #[test]
    fn test_steam_checker_parse_response() {
        let checker = SteamChecker::new();
        assert_eq!(checker.parse_response(200, None), Some(true));
        assert_eq!(checker.parse_response(404, None), Some(false));
        assert_eq!(checker.parse_response(500, None), None);
    }
}
