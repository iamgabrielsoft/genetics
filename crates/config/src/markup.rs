use std::sync::Arc;

use libs::syntect::{
    highlighting::{Theme, ThemeSet},
};
use crate::config_highlight::{ THEME_SET };
use serde::{Deserialize, Serialize};


pub const DEFAULT_HIGHLIGHT_THEME: &str = "base16-ocean-dark";


#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Markdown {
    /// Whether to highlight code blocks
    pub highlight_code: bool,

    pub highlight_theme: String,

    /// Whether to render emojis
    pub render_emoji: bool,

    #[serde(skip_serializing, skip_deserializing)]
    pub extra_theme_set: Arc<Option<ThemeSet>>,
}

impl Markdown {
    pub fn new() -> Self {
        Self {
            highlight_code: false,
            render_emoji: false,
            highlight_theme: DEFAULT_HIGHLIGHT_THEME.to_owned(),
            extra_theme_set: Arc::new(None)
        }
    }

    pub fn get_highlight_theme(&self) -> Option<&Theme>{
        if self.highlight_theme == "css" {
            None
        }
        else {
            self.highlight_theme_by_name(&self.highlight_theme)
        }
    }

    pub fn highlight_theme_by_name(&self, theme_name: &str) -> Option<&Theme>{
        (*self.extra_theme_set).as_ref().and_then(|mx|
        mx.themes.get(theme_name))
        .or_else(|| THEME_SET.themes.get(theme_name))
    }
}


impl Default for Markdown {
    fn default() -> Self {
        Self {
            highlight_code: false,
            render_emoji: false,
            highlight_theme: DEFAULT_HIGHLIGHT_THEME.to_owned(),
            extra_theme_set: Arc::new(None),
        }
    }
}