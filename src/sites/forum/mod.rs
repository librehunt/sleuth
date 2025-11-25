//! Forum and discussion platform site checkers

pub mod reddit;

use crate::sites::Site;

/// Get all forum sites
pub fn all_sites() -> Vec<Box<dyn Site>> {
    vec![Box::new(reddit::RedditChecker::new())]
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
