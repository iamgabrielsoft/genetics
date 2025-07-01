use std::{net::IpAddr, path::PathBuf};
use clap::{Parser, Subcommand};


#[derive(Subcommand)]
pub enum Command {
    /// Scafold a new project
    Init {

        #[clap(default_value = ".")]
        name: String,

        /// force project creation
        #[clap(short = 'f', long)]
        force: bool,
    },


    /// Removes the output directory if it exists and rebuilds the site
    Build {
        base_url: Option<String>, 

        output_dir: Option<PathBuf>,
    },


    /// Serve site, for development, reloading should be automatic
    Serve {

        interface: IpAddr, 

        port: u16,
        
        output_dir: PathBuf,

        base_url: String,
    }
}


#[derive(Parser)]
pub struct Cli {

    #[clap(short = 'r', long, default_value = ".")]
    pub root:PathBuf,


    #[clap(short = 'c', long, default_value = "config.toml")]
    pub config: PathBuf, 

    #[clap(subcommand)]
    pub command: Command, 
}

