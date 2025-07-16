use std::{self, io::{self, BufRead, Write}, path::Path};
use std::fs::{ create_dir };
use std::time::Instant;
use anyhow::{ Result};
use cli::{ Cli, Command };


use crate::utils::{fs::{ build_output_dir, create_file, get_current_config_path }, net::{available_port_checker, serve_site}};

mod cli;
mod utils;


/// Read a line from stdin
fn read_line() -> Result<String, String> {
    let stdin = io::stdin().lock().lines(); 
    let mut lines = stdin;

    lines.next()
        .and_then(|l| l.ok())
        .ok_or_else(|| "No input".to_string())
}

/// Ask a yes/no question
fn ask_bool(question: &str, default: bool) -> Result<bool, String> {
    let _ = io::stdout().flush();
    let input = read_line()?;

    match &*input {
        "y" | "Y" | "yes" | "YES" | "true" => Ok(true),
        "n" | "N" | "no" | "NO" | "false" => Ok(false),
        "" => Ok(default),
        _ => {
            println!("Invalid choice: '{}'", input);
            ask_bool(question, default)
        }
    }
}

/// Ask a URL question
fn ask_url(question: &str, default: &str) -> Result<String, String> {
    print!("{} [{}]", question, default); 
    io::stdout().flush().map_err(|e| e.to_string())?;

    let input = read_line()?.trim().to_string(); 

    match input.as_str() {
        "" => Ok(default.to_string()),
        url if url.starts_with("http://") || url.starts_with("https://") => Ok(url.to_string()),
        _ => {
            println!("Invalid URL: '{}'", input);
            ask_url(question, default)
        }
    }
}


const CONFIG: &str = r#"
    # The URL the site will be built for
    base_url = "%BASE_URL%"

    [extra]
    # All variables should be added here
"#;

fn create_new_project(name: &str, force: bool) -> Result<(), String> {
    let path = Path::new(name); 

    if path.exists() && !force {
        if name == "."  {
            println!("Current directory already exists");
        }
        else {
            println!("Directory {} already exists", path.to_string_lossy().to_string());
        }
    }

    println!("Creating project {}", path.to_string_lossy().to_string());
    println!("Please enter some information about your project");

    let base_url = ask_url("> Enter your website's base URL", "https://xample.com")?;

    let config = CONFIG
        .trim_start()
        .replace("%BASE_URL%", &base_url);

    populate_project(path, &config);

    println!();

    Ok(())
    
}


/// Populates the project directory with the default files
fn populate_project(path: &Path, config: &str) -> Result<()>{
    if !path.exists() {
        // create directory
        create_dir(path)?;
    }

    create_file(&path.join("config.toml"), config)?;
    create_dir(&path.join("content"))?;
    create_dir(&path.join("static"))?;
    create_dir(&path.join("templates"))?;

    Ok(())
}

fn main() {
    let cli = <Cli as clap::Parser>::parse();
    let current_dir = cli.root.canonicalize().unwrap_or_else(|e| {
        std::process::exit(1); 
    });

    match cli.command {
        Command::Init { name, force, .. } => {
            if let Err(e) = create_new_project(&name, force) {
                println!("Unable to create project {}", &e);
                std::process::exit(1)
            }
        }

        Command::Build { base_url, output_dir } => {
            println!("\x1B[1;34m   \x1B[0m Building starting...");
            let start = Instant::now(); 
            let (root_dir, config_file) = get_current_config_path(&cli.root, &cli.config);

            match build_output_dir(&root_dir, &config_file, output_dir.as_deref(), false) {
                Ok(()) => println!("\x1B[1;32m   \x1B[0m Built successfully in {:?}", start.elapsed()),
                Err(e) => {
                    println!("Unable to build output directory: {}", &e);
                    std::process::exit(1);
                }
            }
        }

        Command::Serve { 
            interface, 
            mut port, 
            output_dir, 
            base_url,
            open,
            // no_port_append,
        } => {
            //when port is not 1111, check if it is available
            if port != 8080 && !available_port_checker(interface, port) {
                println!("Port {} is not available", port);
                std::process::exit(1); 
            }


            let (root_dir, config_file) = get_current_config_path(&current_dir, &cli.config); 
            println!("\x1B[1;34m   \x1B[0m Serving starting..."); 
            if let Err(err) = serve_site(
                &root_dir,
                interface,
                port,
                output_dir.as_deref(),
                false,
                base_url.as_deref(),
                &config_file,
                false,
                false,
            ) {
                println!("Unable to serve site: {}", &err);
                std::process::exit(1);
            }
        }
    }
    
}
