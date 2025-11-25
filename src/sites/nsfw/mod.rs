//! NSFW/adult content platform site checkers

pub mod onlyfans;

use crate::sites::Site;

/// Get all NSFW sites
pub fn all_sites() -> Vec<Box<dyn Site>> {
    vec![Box::new(onlyfans::OnlyFansChecker::new())]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nsfw_sites_module() {
        // NSFW sites module tests
        assert!(true);
    }
}
