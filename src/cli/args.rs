//! Argument parsing for CLI

use crate::data::site_info::SiteType;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "sleuth")]
#[command(about = "A Rust implementation of sherlock")]
#[command(version)]
pub struct Args {
    /// Username to search for
    #[arg(required = true)]
    pub username: String,

    /// Filter by site type (dev, social, nsfw, professional, gaming, forum, other)
    /// Can be specified multiple times to include multiple types
    #[arg(long = "type", short = 't', value_name = "TYPE")]
    pub site_types: Vec<String>,

    /// Filter by specific site names
    /// Can be specified multiple times
    #[arg(long = "site", short = 's', value_name = "SITE")]
    pub sites: Vec<String>,

    /// Output format: text, json, csv
    #[arg(long = "format", short = 'f', default_value = "text")]
    pub output_format: String,

    /// Timeout in seconds for each request
    #[arg(long = "timeout", default_value_t = 10)]
    pub timeout: u64,

    /// Number of retries for failed requests
    #[arg(long = "retries", default_value_t = 3)]
    pub retries: u32,

    /// Verify found results with browser headless rendering
    /// 
    /// By default, sleuth uses fast HTTP HEAD requests for all sites. When --verify is enabled,
    /// a second pass is performed: sites that were found in the first pass and require browser
    /// rendering (e.g., JavaScript-heavy sites like OnlyFans, Reddit) are verified using a
    /// headless browser to ensure accuracy and eliminate false positives.
    /// 
    /// This is slower but more accurate, especially for sites that render content dynamically.
    #[arg(long = "verify")]
    pub verify: bool,
}

impl Args {
    /// Parse site types from string arguments
    pub fn parsed_site_types(&self) -> Vec<SiteType> {
        self.site_types
            .iter()
            .filter_map(|s| SiteType::parse(s))
            .collect()
    }

    /// Check if site type filtering is enabled
    pub fn has_type_filter(&self) -> bool {
        !self.site_types.is_empty()
    }

    /// Check if site name filtering is enabled
    pub fn has_site_filter(&self) -> bool {
        !self.sites.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_parsed_site_types() {
        let args = Args {
            username: "test".to_string(),
            site_types: vec!["dev".to_string(), "social".to_string()],
            sites: vec![],
            output_format: "text".to_string(),
            timeout: 10,
            retries: 3,
            verify: false,
        };
        let types = args.parsed_site_types();
        assert_eq!(types.len(), 2);
        assert!(types.contains(&SiteType::Dev));
        assert!(types.contains(&SiteType::Social));
    }

    #[test]
    fn test_args_parsed_site_types_invalid() {
        let args = Args {
            username: "test".to_string(),
            site_types: vec!["dev".to_string(), "invalid".to_string()],
            sites: vec![],
            output_format: "text".to_string(),
            timeout: 10,
            retries: 3,
            verify: false,
        };
        let types = args.parsed_site_types();
        assert_eq!(types.len(), 1);
        assert!(types.contains(&SiteType::Dev));
    }

    #[test]
    fn test_args_has_type_filter() {
        let args = Args {
            username: "test".to_string(),
            site_types: vec!["dev".to_string()],
            sites: vec![],
            output_format: "text".to_string(),
            timeout: 10,
            retries: 3,
            verify: false,
        };
        assert!(args.has_type_filter());

        let args_no_filter = Args {
            username: "test".to_string(),
            site_types: vec![],
            sites: vec![],
            output_format: "text".to_string(),
            timeout: 10,
            retries: 3,
            verify: false,
        };
        assert!(!args_no_filter.has_type_filter());
    }

    #[test]
    fn test_args_has_site_filter() {
        let args = Args {
            username: "test".to_string(),
            site_types: vec![],
            sites: vec!["github".to_string()],
            output_format: "text".to_string(),
            timeout: 10,
            retries: 3,
            verify: false,
        };
        assert!(args.has_site_filter());

        let args_no_filter = Args {
            username: "test".to_string(),
            site_types: vec![],
            sites: vec![],
            output_format: "text".to_string(),
            timeout: 10,
            retries: 3,
            verify: false,
        };
        assert!(!args_no_filter.has_site_filter());
    }
}
