use std::ops::RangeInclusive;


#[derive(Debug)]
pub struct FenceSettings<'a> {
    pub name: Option<&'a str>,
    pub hide_lines: Vec<RangeInclusive<usize>>,
    pub highlight_lines: Vec<RangeInclusive<usize>>,
    pub line_number_start: usize, 
    pub line_numbers: bool,
    pub language: Option<&'a str>,
}

#[derive(Debug)]
pub enum FenceToken<'a> {
    /// Enable line numbers
    EnableLineNumbers,
    
    /// Initial line number. Useful for setting the starting line number for line numbers
    InitialNumber(usize), 
    
    /// Inclusive range of line numbers. Useful for highlighting or hiding ranges of lines
    HideLines(Vec<RangeInclusive<usize>>), 
    
    /// Inclusive range of line numbers. Useful for highlighting or hiding ranges of lines
    HighlightLine(Vec<RangeInclusive<usize>>),

    /// Name of the fence
    Name(&'a str),

    /// Language of the fence
    Language(&'a str),
}

struct FenceIter<'a> {
    split: std::str::Split<'a, char>
}

impl<'a> FenceIter<'a> {
    /// Creates a new iterator over the fence info
    fn new(fence_info: &'a str) -> Self {
        Self {
            split: fence_info.split(',')
        }
    }

    fn parse_ranges(token: Option<&'a str>) -> Vec<RangeInclusive<usize>> {
        let mut ranges = Vec::new(); 

        for range in token.unwrap_or("").split(' ') {
            if let Some(range) = Self::parse_range(range) {
                ranges.push(range);
            }
        } 

        ranges
    }

    /// Parses a range of line numbers
    fn parse_range(s: &str) -> Option<RangeInclusive<usize>> {
        match s.find('-') {
            Some(pos) => {
                let mut from = s[..pos].parse().ok()?; 
                let mut to = s[pos + 1..].parse().ok()?; 
                if to < from {
                    std::mem::swap(&mut from, &mut to);
                }
                Some(from..=to)
            }
            None => {
                let val = s.parse().ok()?; 
                // Convert the string to a usize and create a range that includes the converted value
                Some(val..=val)
            }
        }
    }
}

impl<'a> FenceSettings<'a> {
    pub fn new(fence_info: &'a str) -> Self {
        let mut init = Self  {
            name: None,
            hide_lines: Vec::new(),
            highlight_lines: Vec::new(),
            line_number_start: 1,
            line_numbers: false,
            language: None,
        };


        for token in FenceIter::new(fence_info) {
            match token {
                FenceToken::EnableLineNumbers => init.line_numbers = true, 
                FenceToken::InitialNumber(num) => init.line_number_start = num, 
                FenceToken::HideLines(lines) => init.hide_lines = lines, 
                FenceToken::HighlightLine(lines) => init.highlight_lines = lines, 
                FenceToken::Name(name) => init.name = Some(name), 
                FenceToken::Language(lang) => init.language = Some(lang), 
            }
        }

        init
    }
}

/// Iterator over the fence info
impl<'a> Iterator for FenceIter<'a>{
    type Item = FenceToken<'a>;

    fn next(&mut self) -> Option<FenceToken<'a>> {
        loop {
            let token  = self.split.next()?.trim(); 

            let mut token_split = token.split('='); 
            match  token_split.next().unwrap_or("").trim() {
                "" => continue,

                "linenostart" => {
                    if let Some(n) = token_split.next() {
                        return Some(FenceToken::InitialNumber(n.parse().unwrap()));
                    }
                },

                "name" => {
                    if let Some(n) = token_split.next() {
                        return Some(FenceToken::Name(n)); 
                    }
                },
                "hide_lines" => {
                    let ranges = Self::parse_ranges(token_split.next());
                    return Some(FenceToken::HideLines(ranges));
                },
                "hl_lines" => {
                    let ranges = Self::parse_ranges(token_split.next());
                    return Some(FenceToken::HighlightLine(ranges));
                },
                "enablelinenumbers" => {
                    return Some(FenceToken::EnableLineNumbers);
                },

                lang => {
                    if token_split.next().is_some() {
                        eprintln!("Invalid fence token: {}", token);
                        continue; 
                    }

                    return Some(FenceToken::Language(lang));
                },                
            }
        }
    }
}