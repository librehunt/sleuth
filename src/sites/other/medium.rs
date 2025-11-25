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
        assert_eq!(checker.http_method(), "HEAD");
    }

    #[test]
    fn test_medium_checker_parse_response() {
        let checker = MediumChecker::new();
        assert_eq!(checker.parse_response(200, None), Some(true));
        assert_eq!(checker.parse_response(404, None), Some(false));
        assert_eq!(checker.parse_response(500, None), None);
    }
}
