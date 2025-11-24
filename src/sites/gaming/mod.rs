//! Gaming platform site checkers

// Site implementations will be added here
// pub mod steam;
// pub mod xbox;

use crate::sites::Site;

/// Get all gaming sites
pub fn all_sites() -> Vec<Box<dyn Site>> {
    vec![
        // Box::new(steam::SteamChecker::new()),
        // Box::new(xbox::XboxChecker::new()),
        // Add more as needed
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gaming_sites_module() {
        // Gaming sites module tests
        assert!(true);
    }
}
