//! Other/uncategorized site checkers

// Site implementations will be added here

use crate::sites::Site;

/// Get all other sites
pub fn all_sites() -> Vec<Box<dyn Site>> {
    vec![
        // Add other site checkers here
    ]
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
