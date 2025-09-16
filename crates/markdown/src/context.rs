use std::{borrow::Cow, collections::HashMap};
use config::Config;
use tera::{Context as TeraContext, Tera};

#[derive(Debug)]
pub struct RenderContext<'a> {
    pub tera: Cow<'a, Tera>, 
    pub config: &'a Config, 
    pub tera_context: TeraContext,
    pub current_page_path: Option<&'a str>,
    pub current_page_permalink: &'a str,
    pub permalinks: Cow<'a, HashMap<String, String>>,
}

impl<'a> RenderContext<'a> {
    pub fn new(
        tera: &'a Tera,
        config: &'a Config,
        tera_context: TeraContext,
        current_page_path: Option<&'a str>,
        current_page_permalink: &'a str,
        permalinks: Cow<'a, HashMap<String, String>>,
    ) -> Self {
        let mut tera_context = tera_context.clone(); 
        tera_context.insert("config", config); 

        Self {
            tera: Cow::Borrowed(tera),
            config,
            tera_context,
            current_page_path,
            current_page_permalink,
            permalinks,
        }
    }

    

    /// Creates a new RenderContext with default values
    pub fn from_config(config: &'a Config) -> RenderContext<'a>{
        Self {
            tera: Cow::Owned(Tera::default()),
            tera_context: TeraContext::new(),
            config,
            current_page_path: None,
            current_page_permalink: "",
            // Cow::Owned creates a new owned HashMap and wraps it into a `Cow`
            // This is useful when you want to create a default value for a `Cow`
            // that is not borrowed from anywhere, but owned by the `Cow` itself

            permalinks: Cow::Owned(HashMap::new()),
        }
    }
}