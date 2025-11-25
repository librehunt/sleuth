//! Professional network site checkers

pub mod linkedin;

use crate::sites::Site;

/// Get all professional sites
pub fn all_sites() -> Vec<Box<dyn Site>> {
    vec![Box::new(linkedin::LinkedInChecker::new())]
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
