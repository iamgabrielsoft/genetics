use std::{net::{IpAddr, TcpListener}, path::{Path, PathBuf}};
use std::time::Instant;
use anyhow::{ anyhow, Result};
use std::sync::mpsc::channel;
use ctrlc;

use crate::utils::fs::generate_site;
use crate::utils::{fs::create_directory}; 


#[derive(Debug, PartialEq)]
pub enum WatchStatus {
    Required, 
    Optional, 
    Conditional(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum RecursiveMode {
    // watch sub-directories
    Recursive, 

    // watch provided directory
    NonRecursive,
}


/// TODO
pub fn serve_site(
    root_dir: &Path,
    interface: IpAddr,
    interface_port: u16,
    output_dir: Option<&Path>,
    force: bool,
    base_url: Option<&str>,
    config_file: &Path,
    open: bool,
    no_port_append: bool,
) -> Result<()> {
    let start = Instant::now();

    let (mut site, address, base_url) = generate_site(
        root_dir,
        interface,
        interface_port,
        output_dir,
        force,
        base_url,
        config_file,
        no_port_append,
    )?;

    
    if(TcpListener::bind(address)).is_err() {
        return Err(anyhow!("Cannot start server on address {}.", address));
    }

    let config_buf = PathBuf::from(config_file); 
    let root_dir_str = root_dir.to_str().expect("Invalid root directory");

    let mut watch_vector = vec![
        (root_dir_str, WatchStatus::Required, RecursiveMode::NonRecursive),
        ("content", WatchStatus::Required, RecursiveMode::NonRecursive),
        ("static", WatchStatus::Optional, RecursiveMode::Recursive),
        ("templates", WatchStatus::Optional, RecursiveMode::Recursive),
    ];

    //let (tx, rx) = channel();


    //let mut watchers = Vec::new();
    for (entry, mode, recursive_mode) in watch_vector {
        let watch_path = root_dir.join(entry);
        let watch_state = match mode {
            WatchStatus::Required => true,
            WatchStatus::Optional => watch_path.exists(),
            WatchStatus::Conditional(x) => x && watch_path.exists(),
        };
    }

    let output_path = site.output_path;
    create_directory(&output_path)?;

    // let watch_list = watchers
    //     .iter()
    //     .map(|w| if w == root_dir_str { config_buf } else { w })
    //     .collect::<Vec<_>>()
    //     .join(",");

    // println!("\nWatching directories: {}", watch_list);

    println!("Use Ctrl+C to stop\n");

    ctrlc::set_handler(move || {
        println!("\nShutting down...");
        ::std::process::exit(0);
    })
    .expect("Unable to set Ctrl+C handler");

    Ok(())
}

/// Gets an available port
pub fn get_available_port(interface:IpAddr, prevent: u16) -> Option<u16> {
    (1024..9000).find(|port| *port != prevent && available_port_checker(interface, *port))
}

/// Checks if a port is available
pub fn available_port_checker(interface:IpAddr, port: u16) -> bool {
    TcpListener::bind((interface, port)).is_ok()
}


#[cfg(test)]
mod test {

}