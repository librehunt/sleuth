//! Development/tech platform site checkers

pub mod github;
// Site implementations will be added here
// pub mod gitlab;
// pub mod bitbucket;

use crate::sites::Site;

pub use github::GitHubChecker;

/// Get all dev sites
pub fn all_sites() -> Vec<Box<dyn Site>> {
    vec![
        Box::new(GitHubChecker::new()),
        // Box::new(gitlab::GitLabChecker::new()),
        // Add more as needed
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dev_sites_module() {
        // Dev sites module tests
        assert!(true);
    }
}
