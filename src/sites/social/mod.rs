//! Social media platform site checkers

pub mod twitter;

use crate::sites::Site;

/// Get all social sites
pub fn all_sites() -> Vec<Box<dyn Site>> {
    vec![Box::new(twitter::TwitterChecker::new())]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_social_sites_module() {
        // Social sites module tests
        assert!(true);
    }
}
