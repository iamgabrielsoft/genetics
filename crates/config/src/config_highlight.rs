use libs::once_cell::sync::Lazy;
use libs::syntect::parsing::{SyntaxReference, SyntaxSet};
use libs::syntect::highlighting::{Theme, ThemeSet};

use crate::Config;

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
