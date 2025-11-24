//! Site registry for managing all sites

use crate::data::site_info::SiteType;
use crate::sites::Site;
use std::collections::HashMap;

/// Registry for managing all available sites
pub struct SiteRegistry {
    /// All sites in order
    all_sites: Vec<Box<dyn Site>>,
}

impl SiteRegistry {
    /// Create a new site registry with all registered sites
    pub fn new() -> Self {
        let all_sites = crate::sites::all_sites();
        Self::from_sites(all_sites)
    }

    /// Create a registry from a list of sites
    pub fn from_sites(sites: Vec<Box<dyn Site>>) -> Self {
        Self { all_sites: sites }
    }

    /// Get all sites
    pub fn all(&self) -> &[Box<dyn Site>] {
        &self.all_sites
    }

    /// Get sites by type
    pub fn by_type(&self, site_type: SiteType) -> Vec<&dyn Site> {
        self.all_sites
            .iter()
            .filter(|site| site.site_type() == site_type)
            .map(|site| site.as_ref())
            .collect()
    }

    /// Get sites by multiple types
    pub fn by_types(&self, types: &[SiteType]) -> Vec<&dyn Site> {
        if types.is_empty() {
            return self.all_sites.iter().map(|s| s.as_ref()).collect();
        }

        let type_set: std::collections::HashSet<SiteType> = types.iter().copied().collect();
        self.all_sites
            .iter()
            .filter(|site| type_set.contains(&site.site_type()))
            .map(|site| site.as_ref())
            .collect()
    }

    /// Find a site by name (case-insensitive)
    pub fn by_name(&self, name: &str) -> Option<&dyn Site> {
        let name_lower = name.to_lowercase();
        self.all_sites
            .iter()
            .find(|site| site.name().to_lowercase() == name_lower)
            .map(|site| site.as_ref())
    }

    /// Get sites by names (case-insensitive)
    pub fn by_names(&self, names: &[String]) -> Vec<&dyn Site> {
        if names.is_empty() {
            return self.all_sites.iter().map(|s| s.as_ref()).collect();
        }

        let name_set: std::collections::HashSet<String> =
            names.iter().map(|s| s.to_lowercase()).collect();
        self.all_sites
            .iter()
            .filter(|site| name_set.contains(&site.name().to_lowercase()))
            .map(|site| site.as_ref())
            .collect()
    }

    /// Filter sites by both type and name
    pub fn filter(&self, types: &[SiteType], names: &[String]) -> Vec<&dyn Site> {
        let mut filtered: Vec<&dyn Site> = if types.is_empty() {
            self.all_sites.iter().map(|s| s.as_ref()).collect()
        } else {
            self.by_types(types)
        };

        if !names.is_empty() {
            let name_set: std::collections::HashSet<String> =
                names.iter().map(|s| s.to_lowercase()).collect();
            filtered.retain(|site| name_set.contains(&site.name().to_lowercase()));
        }

        filtered
    }

    /// Get count of all sites
    pub fn count(&self) -> usize {
        self.all_sites.len()
    }

    /// Get count of sites by type
    pub fn count_by_type(&self, site_type: SiteType) -> usize {
        self.by_type(site_type).len()
    }

    /// Get statistics about sites
    pub fn statistics(&self) -> SiteStatistics {
        let mut counts_by_type = HashMap::new();
        for site_type in SiteType::all() {
            counts_by_type.insert(site_type, self.count_by_type(site_type));
        }

        SiteStatistics {
            total: self.count(),
            by_type: counts_by_type,
        }
    }
}

impl Default for SiteRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about registered sites
#[derive(Debug, Clone)]
pub struct SiteStatistics {
    /// Total number of sites
    pub total: usize,
    /// Count of sites by type
    pub by_type: HashMap<SiteType, usize>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::result::SearchResult;
    use crate::sites::Site;
    use crate::utils::error::Result;
    use async_trait::async_trait;

    // Mock site for testing
    struct MockSite {
        name: String,
        site_type: SiteType,
    }

    #[async_trait]
    impl Site for MockSite {
        fn name(&self) -> &str {
            &self.name
        }

        async fn check_username(&self, _username: &str) -> Result<SearchResult> {
            todo!()
        }

        fn url_pattern(&self) -> &str {
            ""
        }

        fn site_type(&self) -> SiteType {
            self.site_type
        }
    }

    #[test]
    fn test_registry_new() {
        let registry = SiteRegistry::new();
        // Should work even with empty sites
        assert!(registry.count() >= 0);
    }

    #[test]
    fn test_registry_from_sites() {
        let sites: Vec<Box<dyn Site>> = vec![
            Box::new(MockSite {
                name: "GitHub".to_string(),
                site_type: SiteType::Dev,
            }),
            Box::new(MockSite {
                name: "Twitter".to_string(),
                site_type: SiteType::Social,
            }),
        ];
        let registry = SiteRegistry::from_sites(sites);
        assert_eq!(registry.count(), 2);
    }

    #[test]
    fn test_registry_by_type() {
        let sites: Vec<Box<dyn Site>> = vec![
            Box::new(MockSite {
                name: "GitHub".to_string(),
                site_type: SiteType::Dev,
            }),
            Box::new(MockSite {
                name: "Twitter".to_string(),
                site_type: SiteType::Social,
            }),
        ];
        let registry = SiteRegistry::from_sites(sites);
        let dev_sites = registry.by_type(SiteType::Dev);
        assert_eq!(dev_sites.len(), 1);
        assert_eq!(dev_sites[0].name(), "GitHub");
    }

    #[test]
    fn test_registry_by_name() {
        let sites: Vec<Box<dyn Site>> = vec![Box::new(MockSite {
            name: "GitHub".to_string(),
            site_type: SiteType::Dev,
        })];
        let registry = SiteRegistry::from_sites(sites);
        let site = registry.by_name("github");
        assert!(site.is_some());
        assert_eq!(site.unwrap().name(), "GitHub");
    }

    #[test]
    fn test_registry_filter() {
        let sites: Vec<Box<dyn Site>> = vec![
            Box::new(MockSite {
                name: "GitHub".to_string(),
                site_type: SiteType::Dev,
            }),
            Box::new(MockSite {
                name: "Twitter".to_string(),
                site_type: SiteType::Social,
            }),
        ];
        let registry = SiteRegistry::from_sites(sites);
        let filtered = registry.filter(&[SiteType::Dev], &["github".to_string()]);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name(), "GitHub");
    }

    #[test]
    fn test_registry_statistics() {
        let sites: Vec<Box<dyn Site>> = vec![
            Box::new(MockSite {
                name: "GitHub".to_string(),
                site_type: SiteType::Dev,
            }),
            Box::new(MockSite {
                name: "Twitter".to_string(),
                site_type: SiteType::Social,
            }),
        ];
        let registry = SiteRegistry::from_sites(sites);
        let stats = registry.statistics();
        assert_eq!(stats.total, 2);
        assert_eq!(stats.by_type.get(&SiteType::Dev), Some(&1));
        assert_eq!(stats.by_type.get(&SiteType::Social), Some(&1));
    }
}
