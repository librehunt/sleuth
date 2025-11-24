//! Sleuth - A Rust implementation of sherlock
//!
//! Sleuth is an OSINT (Open Source Intelligence) tool for finding usernames
//! across social networks.

pub mod cli;
pub mod config;
pub mod core;
pub mod data;
pub mod http;
pub mod sites;
pub mod utils;

// Re-export commonly used types
pub use core::result::SearchResult;
pub use data::site_info::{SiteInfo, SiteType};
pub use sites::Site;
pub use utils::error::{Result, SleuthError};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_initialization() {
        // Basic smoke test
        assert!(true);
    }
}
