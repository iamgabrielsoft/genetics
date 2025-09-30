use libs::once_cell::sync::Lazy;
use libs::syntect::parsing::{SyntaxReference, SyntaxSet};
use libs::syntect::highlighting::{Theme, ThemeSet};
use std::collections::HashMap;

use crate::Config;

/// A set of additional themes that can be used for syntax highlighting
#[derive(Default, Debug, Clone)]
pub struct ExtraThemeSet {
    pub themes: HashMap<String, Theme>,
}

pub static SYNTAX_SET: Lazy<SyntaxSet> = Lazy::new(SyntaxSet::load_defaults_newlines);
pub static THEME_SET: Lazy<ThemeSet> = Lazy::new(ThemeSet::load_defaults);

#[derive(Debug, Clone)]
pub enum HighlightStyle {
    FOLLOWED, 

    None,
    
}

pub struct SyntaxTheme<'config> {
    pub syntax: &'config SyntaxReference,
    pub theme: Option<&'config Theme>,
    pub style: HighlightStyle,
    pub syntax_set: &'config SyntaxSet,
}

impl<'config> Default for SyntaxTheme<'config> {
    fn default() -> Self {
        // Use the default syntax set and plain text syntax as fallback
        let syntax_set = &SYNTAX_SET;
        let syntax = syntax_set.find_syntax_plain_text();
        
        Self {
            syntax,
            theme: None,
            style: HighlightStyle::None,
            syntax_set,
        }
    }
}


pub fn fix_highlighting<'config>(language: Option<&str>, config: &'config Config) -> SyntaxTheme<'config> {
    // We need to get the configured theme
    let theme = config.markdown.get_highlight_theme();
    
    if let Some(lang) = language {
        let capture_js= if lang == "js" || lang == "javascript" {
            "ts"
        }
        else { lang };
        if let Some(syntax) = SYNTAX_SET.find_syntax_by_token(capture_js) {
            SyntaxTheme {
                syntax,
                syntax_set: &SYNTAX_SET as &SyntaxSet,
                theme,
                style: HighlightStyle::FOLLOWED,
            }
        }
        else {
            SyntaxTheme {
                syntax: SYNTAX_SET.find_syntax_plain_text(),
                syntax_set: &SYNTAX_SET as &SyntaxSet,
                theme,
                style: HighlightStyle::None,
            }
        }
    }
    else {
        SyntaxTheme {
            syntax: SYNTAX_SET.find_syntax_plain_text(),
            theme, 
            style: HighlightStyle::None,
            syntax_set: &SYNTAX_SET,
        }
    }
}
