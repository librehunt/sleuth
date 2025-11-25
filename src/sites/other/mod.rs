//! Other/uncategorized site checkers

pub mod medium;

use crate::sites::Site;

/// Get all other sites
pub fn all_sites() -> Vec<Box<dyn Site>> {
    vec![Box::new(medium::MediumChecker::new())]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_other_sites_module() {
        // Other sites module tests
        assert!(true);
    }
}
