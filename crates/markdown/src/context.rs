use std::{borrow::Cow};
use config::Config;
use libs::tera::{Context as TeraContext, Tera};

#[derive(Debug)]
pub struct RenderContext<'a> {
    pub tera: Cow<'a, Tera>, 
    pub config: &'a Config, 
    pub tera_context: TeraContext,
}

impl<'a> RenderContext<'a> {
    pub fn new(
        tera: &'a Tera,
        config: &'a Config,
        tera_context: TeraContext,
    ) -> Self {
        let mut tera_context = tera_context.clone(); 
        tera_context.insert("config", config); 

        Self {
            tera: Cow::Borrowed(tera),
            config,
            tera_context,
        }
    }

    

    pub fn from_config(config: &'a Config) -> RenderContext<'a>{
        Self {
            tera: Cow::Owned(Tera::default()),
            tera_context: TeraContext::new(),
            config,

        }
    }
}