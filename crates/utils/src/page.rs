use std::path::Path;
use regex::Regex; 
use anyhow::{Ok, Result, Error}; 
use once_cell::sync::Lazy;

use crate::fs::read_file;
use crate::site::Config;

static TOML_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"^[[:space:]]*\+\+\+(\r?\n(?s).*?(?-s))\+\+\+[[:space:]]*(?:$|(?:\r?\n((?s).*(?-s))$))",
    )
    .unwrap()
});

#[allow(dead_code)]
pub enum FrontMatter<'a> {
    Toml(&'a str),
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Page {
    pub file: String,
    pub content: String,
    
}

impl Page {
    /// Create a new page from a file
    pub fn new<P: AsRef<Path>>(file_path: P, base_path: &Path) -> Page{
        let file_path = file_path.as_ref(); 

        Page {
            file: file_path.display().to_string(),
            ..Self::default()
        }
    }

    /// Breakdown files for front matter
    fn split_content<'a>(file_path: &Path, content: &'a str) -> Result<(FrontMatter<'a>, &'a str), Error> {
        let regex = if TOML_REGEX.is_match(content) {
            &TOML_REGEX
        } else {
            return Err(anyhow::anyhow!("Invalid front matter format in {}", file_path.display()));
        };

        let toml_content = regex.captures(content)
            .ok_or_else(|| anyhow::anyhow!("Failed to capture TOML content in {}", file_path.display()))?;
            
        let front_matter = toml_content.get(1)
            .ok_or_else(|| anyhow::anyhow!("No front matter found in {}", file_path.display()))?
            .as_str();

        Ok((FrontMatter::Toml(front_matter), content))
    }

    pub fn split_page_content<'a>(file_path: &Path, content: &'a str) -> Result<(FrontMatter<'a>, &'a str), Error> {
        let (front_matter, content) = Self::split_content(file_path, content)?;
        Ok((front_matter, content))
    }


    fn parse(
        file_path: &Path,
        content: &str,
        // config: &Config,
        base_path: &Path,
    ) -> Result<Page> {
        let (front_matter, content) = Self::split_page_content(file_path, content)?;
        let mut page = Self::new(file_path, base_path); 

        page.content = content.to_string(); 
        

        Ok(page)
    }

    /// Read .md files
    pub fn parse_file<P: AsRef<Path>>(file_path: P, config: &Config, base_path: &Path) -> Result<Page> {
        let path = file_path.as_ref(); 
        let content = read_file(path)?; 
        let page = Self::parse(path, &content, base_path)?; 
        
        Ok(page)
    }

    pub fn render_markdown() {}

}