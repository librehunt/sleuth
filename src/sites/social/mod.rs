//! Social media platform site checkers

// Site implementations will be added here
// pub mod twitter;
// pub mod instagram;
// pub mod facebook;

use crate::sites::Site;

/// Get all social sites
pub fn all_sites() -> Vec<Box<dyn Site>> {
    vec![
        // Box::new(twitter::TwitterChecker::new()),
        // Box::new(instagram::InstagramChecker::new()),
        // Add more as needed
    ]
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
