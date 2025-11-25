use crate::data::site_info::SiteType;
use crate::sites::Site;

/// LinkedIn username checker
pub struct LinkedInChecker;

impl LinkedInChecker {
    pub fn new() -> Self {
        Self
    }
}

impl Default for LinkedInChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl Site for LinkedInChecker {
    fn name(&self) -> &str {
        "LinkedIn"
    }

    fn url_pattern(&self) -> &str {
        "https://www.linkedin.com/in/{}"
    }

    fn site_type(&self) -> SiteType {
        SiteType::Professional
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linkedin_checker_new() {
        let checker = LinkedInChecker::new();
        assert_eq!(checker.name(), "LinkedIn");
    }

    #[test]
    fn test_linkedin_checker_url_pattern() {
        let checker = LinkedInChecker::new();
        assert_eq!(checker.url_pattern(), "https://www.linkedin.com/in/{}");
    }

    #[test]
    fn test_linkedin_checker_build_url() {
        let checker = LinkedInChecker::new();
        assert_eq!(
            checker.build_url("williamhgates"),
            "https://www.linkedin.com/in/williamhgates"
        );
    }

    #[test]
    fn test_linkedin_checker_site_type() {
        let checker = LinkedInChecker::new();
        assert_eq!(checker.site_type(), SiteType::Professional);
    }

    #[test]
    fn test_linkedin_checker_http_method() {
        let checker = LinkedInChecker::new();
        assert_eq!(checker.http_method(), "HEAD");
    }

    #[test]
    fn test_linkedin_checker_parse_response() {
        let checker = LinkedInChecker::new();
        assert_eq!(checker.parse_response("testuser", 200, None), Some(true));
        assert_eq!(checker.parse_response("testuser", 404, None), Some(false));
        assert_eq!(checker.parse_response("testuser", 500, None), None);
    }
}
