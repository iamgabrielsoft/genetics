use serde::{Deserialize, Serialize};



#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Markdown {
    /// Whether to highlight code blocks
    pub highlight_code: bool,

    /// Whether to render emojis
    pub render_emoji: bool,
}

impl Markdown {
    pub fn new() -> Self {
        Self {
            highlight_code: false,
            render_emoji: false,
        }
    }
}


impl Default for Markdown {
    fn default() -> Self {
        Self {
            highlight_code: false,
            render_emoji: false,
        }
    }
}