//! Site scanner logic

use crate::core::result::SearchResult;
use crate::data::site_info::SiteType;
use crate::request::{create_request, Request, RequestType};
use crate::sites::Site;
use crate::utils::error::Result;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::task::JoinSet;

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

/// Scan a username across multiple sites
///
/// # Parameters
/// - `username`: The username to search for
/// - `sites`: List of sites to check
/// - `request`: Optional request implementation (defaults to HTTP if None)
pub async fn scan_username(
    username: &str,
    sites: Vec<&dyn Site>,
    request: Option<Arc<dyn Request>>,
) -> Result<Vec<SearchResult>> {
    // Default to HTTP if no request provided
    let request = request.unwrap_or_else(|| {
        create_request(RequestType::Http, 10).expect("Failed to create default HTTP request")
    });

    let mut tasks: JoinSet<Result<SearchResult>> = JoinSet::new();
    let username = username.to_string();

    // Build URLs and spawn tasks for all sites
    // Extract all needed data before spawning to avoid lifetime issues
    // We need to call parse_response on the site, so we'll use a workaround:
    // For now, we'll use the default parse_response logic (200 = exists, 404 = not found)
    // TODO: Refactor to allow sites to provide custom parse_response logic
    let mut site_data: Vec<(String, String, String, String)> = Vec::new();
    for site in sites {
        let url = site.build_url(&username);
        let site_name = site.name().to_string();
        let method = site.http_method().to_string();
        // Note: We're not using site.parse_response here due to lifetime constraints
        // This is a limitation we'll address in a future refactor
        site_data.push((site_name, url, method, username.clone()));
    }

    // Now spawn tasks with owned data
    for (site_name, url, method, username_clone) in site_data {
        let request = Arc::clone(&request);

        tasks.spawn(async move {
            // Make request using the trait
            let response = request.request(&method, &url).await?;

            // Parse response using default logic (200 = exists, 404 = not found)
            // TODO: Use site.parse_response() once we refactor to avoid lifetime issues
            let exists = match response.status_code {
                200..=299 => Some(true),
                404 => Some(false),
                _ => None, // Uncertain, might need retry
            };

            match exists {
                Some(true) => Ok(SearchResult::found(site_name, username_clone, url)),
                Some(false) => Ok(SearchResult::not_found(site_name, username_clone)),
                None => Ok(SearchResult::not_found(site_name, username_clone)),
            }
        });
    }

    // Collect results
    let mut results = Vec::new();
    while let Some(res) = tasks.join_next().await {
        match res {
            Ok(Ok(result)) => results.push(result),
            Ok(Err(e)) => {
                eprintln!("Error checking site: {}", e);
            }
            Err(e) => {
                eprintln!("Task error: {}", e);
            }
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::result::SearchResult;
    use crate::data::site_info::SiteType;
    use crate::sites::Site;

    // Mock site for testing
    struct MockSite {
        name: String,
        site_type: SiteType,
    }

    impl Site for MockSite {
        fn name(&self) -> &str {
            &self.name
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
