use std::ops::RangeInclusive;
use libs::syntect::util::LinesWithEndings;
use config::Config;
use errors::Result;

use crate::fence::FenceSettings;

/// A basic syntax highlighter
#[derive(Debug)]
pub struct SyntaxHighlighter<'a> {
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> SyntaxHighlighter<'a> {
    /// Creates a new SyntaxHighlighter instance
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

/// Code block struct
pub struct CodeBlock<'config> {
    _highlighter: SyntaxHighlighter<'config>,
    line_numbers: bool,
    _line_number_start: usize,
    _highlight_lines: Vec<RangeInclusive<usize>>,
    _hide_lines: Vec<RangeInclusive<usize>>,
}


impl<'config> CodeBlock<'config>{
    pub fn new<'fence_info>(
        fence: FenceSettings<'fence_info>,
        _config: &'config Config,
        _path: Option<&'config str>,
    ) -> Result<(Self, String)> {
        Ok((
            Self {
                _highlighter: SyntaxHighlighter::new(),
                line_numbers: fence.line_numbers,
                _line_number_start: fence.line_number_start,
                _highlight_lines: fence.highlight_lines,
                _hide_lines: fence.hide_lines,
            },
            String::new(),
        ))
    }

    pub fn highlight(&mut self, content: &str) -> String {
        let mut buffer = String::new(); 
       // let mark_style = self.high

       if self.line_numbers {
        buffer.push_str("<table><tbody>");
       }

       // let's process the lines
       for (i, _line) in LinesWithEndings::from(content).enumerate() {
            let one_indexed = i +1; 
            let mut skipper = false; 

            for range in &self._hide_lines {
                if range.contains(&one_indexed) {
                    skipper = true;
                    break; 
                }
            }

            if skipper {
                continue;
            }


            if self.line_numbers {
                buffer.push_str("<tr><td>");
              //  let num = format!("{}", self.line_number_start + i);
               // maybe_mark(&mut buffer, &num);
                buffer.push_str("</td><td>");
            }


            // let highlighted_line = self.highlighter.highlight_line(line);
            //  maybe_mark(&mut buffer, &highlighted_line);

            if self.line_numbers {
                buffer.push_str("</td></tr>");
            }
       }
       

       if self.line_numbers {
        buffer.push_str("</tbody></table>");
       }

       buffer
    }
}