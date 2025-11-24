//! Site-specific implementations

pub mod site;

// Site implementations will be added here
// pub mod github;
// pub mod twitter;
// pub mod instagram;

pub use site::Site;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sites_module() {
        // Sites module tests
        assert!(true);
    }
}
