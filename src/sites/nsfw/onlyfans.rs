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
    fn test_onlyfans_checker_http_method() {
        let checker = OnlyFansChecker::new();
        assert_eq!(checker.http_method(), "HEAD");
    }

    #[test]
    fn test_onlyfans_checker_parse_response() {
        let checker = OnlyFansChecker::new();
        assert_eq!(checker.parse_response(200, None), Some(true));
        assert_eq!(checker.parse_response(404, None), Some(false));
        assert_eq!(checker.parse_response(500, None), None);
    }
}
