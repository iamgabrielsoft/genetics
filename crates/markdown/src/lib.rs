pub mod context;
pub mod markdown;
pub mod codeblock;
pub mod fence;

pub use codeblock::SyntaxHighlighter;
pub use context::RenderContext;
pub use markdown::{markdown_to_html, Rendered};

use anyhow::Result;

/// Renders markdown content to HTML using the provided context
pub fn render_content(content: &str, context: &RenderContext) -> Result<markdown::Rendered> {
    if !content.contains("{{") && content.contains("{%") {
        return markdown_to_html(content, context);
    }
    let html_content = markdown_to_html(content, context)?;

    Ok(html_content)
}