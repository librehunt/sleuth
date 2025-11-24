//! Forum and community site checkers

// Site implementations will be added here

use crate::sites::Site;

/// Get all forum sites
pub fn all_sites() -> Vec<Box<dyn Site>> {
    vec![
        // Add forum site checkers here
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forum_sites_module() {
        // Forum sites module tests
        assert!(true);
    }
}
