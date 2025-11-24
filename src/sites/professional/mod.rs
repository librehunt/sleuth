//! Professional network site checkers

// Site implementations will be added here
// pub mod linkedin;

use crate::sites::Site;

/// Get all professional sites
pub fn all_sites() -> Vec<Box<dyn Site>> {
    vec![
        // Box::new(linkedin::LinkedInChecker::new()),
        // Add more as needed
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_professional_sites_module() {
        // Professional sites module tests
        assert!(true);
    }
}
