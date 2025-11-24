//! Site metadata (URL patterns, etc.)

use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Categories for sites
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SiteType {
    /// Development/tech platforms (GitHub, GitLab, etc.)
    Dev,
    /// Social media platforms (Twitter, Instagram, etc.)
    Social,
    /// NSFW/adult content platforms
    Nsfw,
    /// Professional networks (LinkedIn, etc.)
    Professional,
    /// Gaming platforms (Steam, Xbox, etc.)
    Gaming,
    /// Forums and communities
    Forum,
    /// Other/uncategorized
    Other,
}

impl SiteType {
    /// Get all available site types
    pub fn all() -> Vec<SiteType> {
        vec![
            SiteType::Dev,
            SiteType::Social,
            SiteType::Nsfw,
            SiteType::Professional,
            SiteType::Gaming,
            SiteType::Forum,
            SiteType::Other,
        ]
    }

    /// Parse from string (case-insensitive) - convenience method
    pub fn parse(s: &str) -> Option<Self> {
        Self::from_str(s).ok()
    }

    /// Get display name
    pub fn as_str(&self) -> &'static str {
        match self {
            SiteType::Dev => "dev",
            SiteType::Social => "social",
            SiteType::Nsfw => "nsfw",
            SiteType::Professional => "professional",
            SiteType::Gaming => "gaming",
            SiteType::Forum => "forum",
            SiteType::Other => "other",
        }
    }
}

impl FromStr for SiteType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dev" | "development" => Ok(SiteType::Dev),
            "social" => Ok(SiteType::Social),
            "nsfw" => Ok(SiteType::Nsfw),
            "professional" | "pro" => Ok(SiteType::Professional),
            "gaming" | "game" => Ok(SiteType::Gaming),
            "forum" | "forums" => Ok(SiteType::Forum),
            "other" => Ok(SiteType::Other),
            _ => Err(format!("Unknown site type: {}", s)),
        }
    }
}

/// Metadata about a site
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteInfo {
    /// Site name
    pub name: String,
    /// Site type/category
    pub site_type: SiteType,
    /// URL pattern (e.g., "https://github.com/{}")
    pub url_pattern: String,
    /// Whether the site is enabled by default
    pub enabled: bool,
}

impl SiteInfo {
    /// Create a new SiteInfo
    pub fn new(name: String, site_type: SiteType, url_pattern: String) -> Self {
        Self {
            name,
            site_type,
            url_pattern,
            enabled: true,
        }
    }

    /// Create a new SiteInfo with enabled flag
    pub fn with_enabled(
        name: String,
        site_type: SiteType,
        url_pattern: String,
        enabled: bool,
    ) -> Self {
        Self {
            name,
            site_type,
            url_pattern,
            enabled,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_site_type_from_str() {
        use std::str::FromStr;
        assert_eq!(SiteType::from_str("dev"), Ok(SiteType::Dev));
        assert_eq!(SiteType::from_str("SOCIAL"), Ok(SiteType::Social));
        assert_eq!(SiteType::from_str("nsfw"), Ok(SiteType::Nsfw));
        assert_eq!(
            SiteType::from_str("professional"),
            Ok(SiteType::Professional)
        );
        assert_eq!(SiteType::from_str("pro"), Ok(SiteType::Professional));
        assert_eq!(SiteType::from_str("gaming"), Ok(SiteType::Gaming));
        assert_eq!(SiteType::from_str("game"), Ok(SiteType::Gaming));
        assert_eq!(SiteType::from_str("forum"), Ok(SiteType::Forum));
        assert_eq!(SiteType::from_str("forums"), Ok(SiteType::Forum));
        assert_eq!(SiteType::from_str("other"), Ok(SiteType::Other));
        assert!(SiteType::from_str("invalid").is_err());
    }

    #[test]
    fn test_site_type_as_str() {
        assert_eq!(SiteType::Dev.as_str(), "dev");
        assert_eq!(SiteType::Social.as_str(), "social");
        assert_eq!(SiteType::Nsfw.as_str(), "nsfw");
        assert_eq!(SiteType::Professional.as_str(), "professional");
        assert_eq!(SiteType::Gaming.as_str(), "gaming");
        assert_eq!(SiteType::Forum.as_str(), "forum");
        assert_eq!(SiteType::Other.as_str(), "other");
    }

    #[test]
    fn test_site_type_all() {
        let all = SiteType::all();
        assert_eq!(all.len(), 7);
        assert!(all.contains(&SiteType::Dev));
        assert!(all.contains(&SiteType::Social));
        assert!(all.contains(&SiteType::Nsfw));
        assert!(all.contains(&SiteType::Professional));
        assert!(all.contains(&SiteType::Gaming));
        assert!(all.contains(&SiteType::Forum));
        assert!(all.contains(&SiteType::Other));
    }

    #[test]
    fn test_site_info_new() {
        let info = SiteInfo::new(
            "GitHub".to_string(),
            SiteType::Dev,
            "https://github.com/{}".to_string(),
        );
        assert_eq!(info.name, "GitHub");
        assert_eq!(info.site_type, SiteType::Dev);
        assert_eq!(info.url_pattern, "https://github.com/{}");
        assert!(info.enabled);
    }

    #[test]
    fn test_site_info_with_enabled() {
        let info = SiteInfo::with_enabled(
            "TestSite".to_string(),
            SiteType::Social,
            "https://test.com/{}".to_string(),
            false,
        );
        assert_eq!(info.name, "TestSite");
        assert_eq!(info.site_type, SiteType::Social);
        assert!(!info.enabled);
    }
}
