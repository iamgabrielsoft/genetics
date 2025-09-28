use std::fmt::Write;
use config::config_highlight::{SyntaxTheme};
use libs::syntect::{
    easy::HighlightLines, 
    highlighting::{Color, Theme}, 
    html::{line_tokens_to_classed_spans, styled_line_to_highlighted_html, ClassStyle, IncludeBackground}, 
    parsing::{ParseState, Scope, ScopeStack, SyntaxReference, SyntaxSet, SCOPE_REPO}
};
use errors::Result;
use libs::tera::escape_html;



pub const CLASS_STYLE: ClassStyle = ClassStyle::SpacedPrefixed { prefix: "z-" };

/// Writes a color to a string buffer
/// 
/// # Arguments
/// * `buffer` - The string buffer to write to
/// * `color` - The color to write
pub fn write_color(buffer: &mut String, color: Color) -> std::fmt::Result {
    if color.a != 0xFF {
        write!(buffer, "rgba({},{},{},{:.3})", color.r, color.g, color.b, f32::from(color.a) / 255.0)?;
    } else {
        write!(buffer, "rgb({},{},{})", color.r, color.g, color.b)?;
    }
    Ok(())
}

/// Converts a Scope to a string of classes
/// 
/// # Arguments
/// * `s` - The string to write to
/// * `scope` - The scope to convert
/// * `style` - The style to use
pub fn scope_to_classes(s: &mut String, scope: Scope, style: ClassStyle) {
    let repo = SCOPE_REPO.lock().unwrap();
    // This line locks the SCOPE_REPO mutex and returns a reference to the underlying Repository object.
    // The Repository object is used to convert Scope atoms to their corresponding string representations.

    for i in 0..(scope.len()) {
        let atom = scope.atom_at(i as usize);
        let atom_s = repo.atom_str(atom);
        if i != 0 {
            s.push(' ')
        }
        match style {
            ClassStyle::Spaced => {}
            ClassStyle::SpacedPrefixed { prefix } => {
                s.push_str(prefix);
            }
            _ => {} // Non-exhaustive
        }
        s.push_str(atom_s);
    }
}


pub struct InlineHighlighter<'config> {
    pub theme: &'config Theme,
    syntax_set: &'config SyntaxSet,
    h: HighlightLines<'config>,
    fg_color: String,
    bg_color: Color,
}

pub struct ClassHighlighter<'config> {
    syntax_set: &'config SyntaxSet,
    parse_state: ParseState,
    scope_stack: ScopeStack,
}


pub(crate) enum SyntaxHighlighter {
    // Inline highlighter
    Inlined,
    // Classed highlighter
    Classed,
    // No highlighting
    NoHighlight,
}

impl<'config> InlineHighlighter<'config> {
    pub fn new(
        syntax: &'config SyntaxReference,
        syntax_set: &'config SyntaxSet,
        theme: &'config Theme,
    ) -> Self {
        let h = HighlightLines::new(syntax, theme); 
        let mut color = String::new(); 
        write_color(&mut color, theme.settings.foreground.unwrap_or(Color::BLACK)).unwrap();
        let fg_color = format!(r#" style="color:{};""#, color);
        let bg_color = theme.settings.background.unwrap_or(Color::WHITE);


        Self {
            theme, 
            syntax_set,
            h,
            fg_color,
            bg_color,
        }
    }

    /// Highlights a line of code
    /// 
    /// # Arguments
    /// * `line` - The line to highlight
    /// 
    /// # Returns
    /// * `Result<String>` - The highlighted line
    pub fn highlight_line(&mut self, line: &str) -> Result<String> {
        let areas = self.h.highlight_line(line, self.syntax_set).expect("Unable to highlight line"); 
        let highlighted = styled_line_to_highlighted_html(
            &areas,
            IncludeBackground::IfDifferent(self.bg_color),
        ).expect("Unable to highlight line");
        
        Ok(highlighted.replace(&self.fg_color, ""))
    }
}

impl<'config> ClassHighlighter<'config> {
    pub fn new(
        syntax: &'config SyntaxReference,
        syntax_set: &'config SyntaxSet,
    ) -> Self {
        let parse_state = ParseState::new(syntax);
        Self {
            parse_state,
            syntax_set,
            scope_stack: ScopeStack::new(),
        }
    }

    /// Highlights a line of code
    /// 
    /// # Arguments
    /// * `line` - The line to highlight
    /// 
    /// # Returns
    /// * `Result<String>` - The highlighted line
    pub fn highlight_line(&mut self, line: &str) -> Result<String> {
        let parsed_line = self.parse_state.parse_line(line, self.syntax_set).expect("Unable to parse line"); 
        let mut formmated_line = String::with_capacity(line.len() + self.scope_stack.len());

        for scope in self.scope_stack.as_slice() {
            formmated_line.push_str("<span class=\"");
            scope_to_classes(&mut formmated_line, *scope, CLASS_STYLE);
            formmated_line.push_str("\">");
        }

        let (formatted_contents, _) = line_tokens_to_classed_spans(
            line,
            parsed_line.as_slice(),
            CLASS_STYLE,
            &mut self.scope_stack,
        )
        .expect("Unable to highlight line");
        
        formmated_line.push_str(&formatted_contents);

        for _ in 0..self.scope_stack.len() {
            formmated_line.push_str("</span>");
        }

        Ok(formmated_line)
    }
}

impl SyntaxHighlighter {
    pub fn new(highlight_code: bool, st: SyntaxTheme) -> Self {
        if highlight_code {
            if st.theme.is_some() {
                SyntaxHighlighter::Inlined
            } else {
                SyntaxHighlighter::Classed
            }
        } else {
            SyntaxHighlighter::NoHighlight 
        }
    }

    /// Highlights a line of code
    /// 
    /// # Arguments
    /// * `line` - The line of code to highlight
    /// 
    /// # Returns
    /// * `Result<String>` - The highlighted line
    #[allow(dead_code)]
    pub fn highlight_line(&mut self, line: &str) -> Result<String> {
        match self {
            SyntaxHighlighter::Inlined => {
                // TODO: Implement inline highlighting if needed
                Ok(escape_html(line))
            }
            SyntaxHighlighter::Classed => {
                // TODO: Implement classed highlighting if needed
                Ok(escape_html(line))
            }
            SyntaxHighlighter::NoHighlight => Ok(escape_html(line)),
        }
    }

    /// Returns the style of the marked code
    /// 
    /// # Returns
    /// * `Option<String>` - The style of the marked code
    /// 
    /// # Examples
    /// 
    /// ```
    /// let style = SyntaxHighlighter::Inlined.marked_style();
    /// assert_eq!(style, None);
    /// ```
    #[allow(dead_code)]
    pub fn marked_style(&self) -> Option<String> {
        match self {
            SyntaxHighlighter::Inlined => {
                // TODO: Return appropriate style if needed
                None
            }
            SyntaxHighlighter::Classed | SyntaxHighlighter::NoHighlight => None,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight_rust() {
        let rust_code = r#"
        fn main() {
            println!("Hello, world!");
        }
        "#;
        
        let highlighted = highlight_code(rust_code, Some("rust")).unwrap();
        assert!(highlighted.contains("fn"));
        assert!(highlighted.contains("main"));
        assert!(highlighted.contains("println!"));
        assert!(highlighted.contains("language-rust"));
    }

    #[test]
    fn test_highlight_unknown_language() {
        let code = "just some plain text";
        let highlighted = highlight_code(code, None).unwrap();
        assert!(highlighted.contains("just some plain text"));
        assert!(highlighted.contains("language-text"));
    }
}
