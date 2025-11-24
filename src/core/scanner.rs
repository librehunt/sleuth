//! Site scanner logic

use crate::data::site_info::SiteType;
use crate::sites::Site;
use std::collections::HashSet;

/// Filter sites by type
pub fn filter_sites_by_type<S: Site>(
    sites: Vec<Box<S>>,
    allowed_types: &[SiteType],
) -> Vec<Box<S>> {
    if allowed_types.is_empty() {
        return sites;
    }

    let type_set: HashSet<SiteType> = allowed_types.iter().copied().collect();
    sites
        .into_iter()
        .filter(|site| type_set.contains(&site.site_type()))
        .collect()
}

/// Filter sites by name
pub fn filter_sites_by_name<S: Site>(sites: Vec<Box<S>>, allowed_names: &[String]) -> Vec<Box<S>> {
    if allowed_names.is_empty() {
        return sites;
    }

    let name_set: HashSet<String> = allowed_names.iter().map(|s| s.to_lowercase()).collect();
    sites
        .into_iter()
        .filter(|site| name_set.contains(&site.name().to_lowercase()))
        .collect()
}

/// Filter sites by both type and name
pub fn filter_sites<S: Site>(
    sites: Vec<Box<S>>,
    allowed_types: &[SiteType],
    allowed_names: &[String],
) -> Vec<Box<S>> {
    let mut filtered = sites;

    if !allowed_types.is_empty() {
        filtered = filter_sites_by_type(filtered, allowed_types);
    }

    if !allowed_names.is_empty() {
        filtered = filter_sites_by_name(filtered, allowed_names);
    }

    filtered
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::result::SearchResult;
    use crate::data::site_info::SiteType;
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

    #[tokio::test]
    async fn test_filter_sites_by_type_empty() {
        let sites: Vec<MockSite> = vec![MockSite {
            name: "GitHub".to_string(),
            site_type: SiteType::Dev,
        }];
        let sites_boxed: Vec<Box<MockSite>> = sites.into_iter().map(Box::new).collect();
        let filtered = filter_sites_by_type(sites_boxed, &[]);
        assert_eq!(filtered.len(), 1);
    }

    #[tokio::test]
    async fn test_filter_sites_by_type_single() {
        let sites: Vec<MockSite> = vec![
            MockSite {
                name: "GitHub".to_string(),
                site_type: SiteType::Dev,
            },
            MockSite {
                name: "Twitter".to_string(),
                site_type: SiteType::Social,
            },
        ];
        let sites_boxed: Vec<Box<MockSite>> = sites.into_iter().map(Box::new).collect();
        let filtered = filter_sites_by_type(sites_boxed, &[SiteType::Dev]);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name(), "GitHub");
    }

    #[tokio::test]
    async fn test_filter_sites_by_type_multiple() {
        let sites: Vec<MockSite> = vec![
            MockSite {
                name: "GitHub".to_string(),
                site_type: SiteType::Dev,
            },
            MockSite {
                name: "Twitter".to_string(),
                site_type: SiteType::Social,
            },
            MockSite {
                name: "LinkedIn".to_string(),
                site_type: SiteType::Professional,
            },
        ];
        let sites_boxed: Vec<Box<MockSite>> = sites.into_iter().map(Box::new).collect();
        let filtered = filter_sites_by_type(sites_boxed, &[SiteType::Dev, SiteType::Social]);
        assert_eq!(filtered.len(), 2);
    }

    #[tokio::test]
    async fn test_filter_sites_by_name() {
        let sites: Vec<MockSite> = vec![
            MockSite {
                name: "GitHub".to_string(),
                site_type: SiteType::Dev,
            },
            MockSite {
                name: "Twitter".to_string(),
                site_type: SiteType::Social,
            },
        ];
        let sites_boxed: Vec<Box<MockSite>> = sites.into_iter().map(Box::new).collect();
        let filtered =
            filter_sites_by_name(sites_boxed, &["github".to_string(), "twitter".to_string()]);
        assert_eq!(filtered.len(), 2);
    }

    #[tokio::test]
    async fn test_filter_sites_by_name_case_insensitive() {
        let sites: Vec<MockSite> = vec![MockSite {
            name: "GitHub".to_string(),
            site_type: SiteType::Dev,
        }];
        let sites_boxed: Vec<Box<MockSite>> = sites.into_iter().map(Box::new).collect();
        let filtered = filter_sites_by_name(sites_boxed, &["GITHUB".to_string()]);
        assert_eq!(filtered.len(), 1);
    }

    #[tokio::test]
    async fn test_filter_sites_combined() {
        let sites: Vec<MockSite> = vec![
            MockSite {
                name: "GitHub".to_string(),
                site_type: SiteType::Dev,
            },
            MockSite {
                name: "Twitter".to_string(),
                site_type: SiteType::Social,
            },
            MockSite {
                name: "GitLab".to_string(),
                site_type: SiteType::Dev,
            },
        ];
        let sites_boxed: Vec<Box<MockSite>> = sites.into_iter().map(Box::new).collect();
        let filtered = filter_sites(sites_boxed, &[SiteType::Dev], &["github".to_string()]);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name(), "GitHub");
    }
}
