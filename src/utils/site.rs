use std::{collections::HashSet, path::{Path, PathBuf}, time::Instant};
use std::sync::{Arc, Mutex, RwLock};
use walkdir::{DirEntry, WalkDir};
use toml;
use serde::{ Deserialize };
use anyhow::{ Result,  bail };
use crate::utils::{fs::{copy_directory, read_file}, page::Page};



const DEFAULT_BASE_URL: &str = "http://localhost:8080";

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// Base URL of the site, the only required config argument
    pub base_url: String,

    /// Title of the site. Defaults to None
    pub title: Option<String>,
    /// Description of the site
    pub description: Option<String>,

    pub output_dir: String,
}

impl Config {
    pub fn from_config_file<P: AsRef<Path>>(path: P) -> Result<Config> {
        let path = path.as_ref(); 
        let config = read_file(path)?; 

        let config = Config::parse(&config)?;
        //let config_dir = path.parent().ok_or_else(|| anyhow::anyhow!("Unable to get config directory"))?; 

        Ok(config)
    }

    pub fn parse(content: &str) -> Result<Config> {
        let config: Config = match toml::from_str(content) {
            Ok(xm) => xm, 
            Err(err) => bail!(err),
        };


        if config.base_url.is_empty() || config.base_url == DEFAULT_BASE_URL {
            bail!("A base URL is required in config.toml with key `base_url`");
        }

        Ok(config)
    }

    /// Parse the config file
    pub fn get_config(filename: &Path) -> Result<Config> {
        Config::from_config_file(filename)
    }
}




#[derive(Debug)]
pub struct Site {
    /// The base path of the site
    pub base_path: PathBuf, 
    /// The config file of the site
    pub config: Config,
    /// The output path of the site
    pub output_path: PathBuf, 

    pub static_path: PathBuf,
}



impl Site {
    pub fn new<P: AsRef<Path>, P2: AsRef<Path>>(path: P, config_file: P2) -> Result<Site> {
        let path = path.as_ref(); 
        let config = Config::get_config(&path.join(config_file))?;
        let output_path = path.join(config.output_dir.clone());
        let static_path = path.join("static");


        // let content_path = path.join("content");
        // let template_path = path.join("templates");
        // let static_path = path.join("static");


        let site = Site{
            base_path: path.to_path_buf(),
            config,
            output_path,
            static_path,
        };


        Ok(site)
    }

    /// Loads all files(markdown, templates, static) from the site
    pub fn load_files(&mut self) -> Result<()> {
        let mut walkdir = WalkDir::new(self.base_path.join("content")).follow_links(true).into_iter(); 

        let mut pages = Vec::new();
        //let mut sections: HashSet<String> = HashSet::new();

        loop {
            let entry = match walkdir.next() {
                None => break, 
                Some(Err(_)) => continue, 
                Some(Ok(entry)) => entry, 
            }; 

            let path = entry.path(); 
            let file_name = match path.file_name() {
                None => continue, 
                Some(name) => name.to_str().unwrap(),
            };

            if file_name.starts_with(".") {
                continue;
            }

            // skip hidden and non .md files in the directory
            if !path.is_dir() && (!file_name.ends_with(".md")) || file_name.starts_with('.') {
                continue; 
            }

            if path.is_dir() {
                let index_files = WalkDir::new(path)
                    .follow_links(true)
                    .max_depth(1)
                    .into_iter()
                    .filter_map(|e| match e {
                        Err(_) => None,
                        Ok(f) => {
                            if f.path().is_file() {
                                Some(f)
                            }
                            else {
                                None
                            }
                        }
                    })
                    .collect::<Vec<DirEntry>>();


                //add sections here
                for index in index_files {
                    let path = index.path(); 
                    
                   // self.add_section(path.display().to_string());
                }
            }
            else {

                //add pages here
                let page = Page::parse_file(path, &self.config, &self.base_path);
                pages.push(page); 
            }
        }

        Ok(())
    }

    /// Add a section to the site
    /// TODO
    pub fn add_section(&mut self, section: HashSet<String>) -> Result<()> {

        
        Ok(())
    }


    pub fn set_output_path<P: AsRef<Path>>(&mut self, path: P) {
        self.output_path = path.as_ref().to_path_buf();
    }

    pub fn copy_static_directories(&self) -> Result<()>{
        if self.static_path.exists() {
            println!("Copying static files from {} to {}", 
                self.static_path.display(), 
                self.output_path.display()
            );
            copy_directory(&self.static_path, &self.output_path)?;
        } else {
            println!("No static directory found at {}", self.static_path.display());
        }
        Ok(())
    }

    /// Build the output directory
    pub fn build_output_dir(&self) -> Result<()> {
        // Create output directory if it doesn't exist
        if !self.output_path.exists() {
            println!("Creating output directory: {}", self.output_path.display());
            std::fs::create_dir_all(&self.output_path)?;
        }

        // Copy static files
        self.copy_static_directories()?;
        
        // Create a simple index.html if it doesn't exist
        // let index_path = self.output_path.join("index.html");
        // if !index_path.exists() {
        //     let content = format!(
        //         r#"<!DOCTYPE html>
        //                 <html>
        //                 <head>
        //                     <title>{}</title>
        //                     <meta charset="utf-8">
        //                 </head>
        //                 <body>
        //                     <h1>Welcome to {}</h1>
        //                     <p>Your site was successfully built!</p>
        //                 </body>
        //                 </html>"#,
        //         self.config.title.as_deref().unwrap_or("My Site"),
        //         self.config.title.as_deref().unwrap_or("My Site")
        //     );
        //     std::fs::write(&index_path, content)?;
        //     println!("Created default index.html");
        // }
        
        Ok(())
    }
}