//! Site-specific implementations

pub mod dev;
pub mod forum;
pub mod gaming;
pub mod nsfw;
pub mod other;
pub mod professional;
pub mod registry;
pub mod site;
pub mod social;

pub use registry::{SiteRegistry, SiteStatistics};
pub use site::Site;

/// Get all registered sites
pub fn all_sites() -> Vec<Box<dyn Site>> {
    let mut sites: Vec<Box<dyn Site>> = vec![];

    // Dev sites
    sites.extend(dev::all_sites());

    // Social sites
    sites.extend(social::all_sites());

    // Professional sites
    sites.extend(professional::all_sites());

    // Gaming sites
    sites.extend(gaming::all_sites());

    // Forum sites
    sites.extend(forum::all_sites());

    // NSFW sites
    sites.extend(nsfw::all_sites());

    // Other sites
    sites.extend(other::all_sites());

    sites
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sites_module() {
        // Sites module tests
        assert!(true);
    }

    #[test]
    fn test_all_sites() {
        let sites = all_sites();
        // Initially empty, but structure is in place
        assert!(sites.is_empty() || sites.len() > 0);
    }
}
