//! Configuration management for the Genetics static site generator.

use serde::{Deserialize, Serialize};

/// Represents the different modes the application can run in
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    /// Normal build mode
    Build,
    /// Check mode (verification without building)
    Check,
    /// Serve mode (run a local server)
    Serve,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Build
    }
}

/// Main configuration structure
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    /// Base URL of the site
    pub base_url: String,
    
    /// Title of the site
    pub title: Option<String>,
    
    /// Description of the site
    pub description: Option<String>,
    
    /// Current operating mode
    pub mode: Mode,
}

/// Serialized version of the config for template rendering
#[derive(Debug, Serialize)]
pub struct SerializedConfig<'a> {
    base_url: &'a str,
    title: Option<&'a str>,
    description: Option<&'a str>,
}

impl Config {
    /// Check if the application is in check mode
    pub fn is_in_check_mode(&self) -> bool {
        self.mode == Mode::Check
    }
    
    /// Enable serve mode
    pub fn enable_serve_mode(&mut self) {
        self.mode = Mode::Serve;
    }
    
    /// Enable check mode
    pub fn enable_check_mode(&mut self) {
        self.mode = Mode::Check;
    }
    
    /// Serialize the config for template rendering
    pub fn serialize(&self) -> SerializedConfig<'_> {
        SerializedConfig {
            base_url: &self.base_url,
            title: self.title.as_deref(),
            description: self.description.as_deref(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_serialization() {
        let config = Config {
            base_url: "https://example.com".to_string(),
            title: Some("Test Site".to_string()),
            description: Some("A test site".to_string()),
            mode: Mode::Build,
        };
        
        let serialized = config.serialize();
        assert_eq!(serialized.base_url, "https://example.com");
        assert_eq!(serialized.title, Some("Test Site"));
        assert_eq!(serialized.description, Some("A test site"));
    }
    
    #[test]
    fn test_mode_switching() {
        let mut config = Config::default();
        
        config.enable_serve_mode();
        assert_eq!(config.mode, Mode::Serve);
        
        config.enable_check_mode();
        assert_eq!(config.mode, Mode::Check);
        assert!(config.is_in_check_mode());
    }
}
