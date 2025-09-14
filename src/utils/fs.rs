use std::io::{Read, Write};

use ws::Sender;
use std::net::{IpAddr, SocketAddr};
use std::path::{ Path, PathBuf };
use std::fs::{ create_dir_all, File };
use errors::{Context, Result, Error};
use walkdir::WalkDir;


use utils::site::Site;


/// Get the current config path
pub fn get_current_config_path(dir: &Path, config_path: &Path) -> (PathBuf, PathBuf) {
    //get the directory ancestors
    let root_dir = dir.ancestors().find(|xm| xm.join(config_path).exists()).unwrap_or_else(|| {
        println!("Unable to locate config file {}", config_path.display());
        std::process::exit(1);
    });

    let config_file_uncontaminated = root_dir.join(config_path);
    let config_file = config_file_uncontaminated.canonicalize().unwrap_or_else(|e|{
        println!("Unable to canonicalize config file {}", config_file_uncontaminated.display());
        std::process::exit(1);
    });


    // (root_dir.join(paths), root_dir.to_path_buf())

    (root_dir.to_path_buf(), config_file)
}

/// Create the parent directory of the given path
fn create_parent(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        create_directory(parent)?;
    }
    Ok(())
}


/// Create the directory if it doesn't exist
pub fn create_directory(path: &Path) -> Result<()>  {
    if !path.exists() {
        create_dir_all(path)
            .with_context(|| format!("Failed to create directory {}", path.display()))?;
    }

    Ok(())
}


/// Create a file from a path
pub fn create_file(path: &Path, content: impl AsRef<str>)  -> Result<()> {
    create_parent(path)?;
    let mut file =
        File::create(path).with_context(|| format!("Failed to create file {}", path.display()))?;
    file.write_all(content.as_ref().as_bytes())?;
    Ok(())
}


/// Get the content of a file with good error handling
pub fn read_file(path: &Path) -> Result<String> {
    let mut content = String::new(); 
    File::open(path)
        .with_context(|| format!("Failed to open file {} ", path.display()))?
        .read_to_string(&mut content)?;

    if content.starts_with('\u{feff}') {
        content.drain(..3);
    }
    
    Ok(content)
}


/// Copy a file to another location
pub fn copy_file(src: &Path, dest: &Path, base_path: &Path)-> Result<()> {
    let relative_path = src.strip_prefix(base_path).unwrap();
    let target_path = dest.join(relative_path);

    create_parent(&target_path)?;

    Ok(())
}


/// Copy a directory to another location
pub fn copy_directory(src: &Path, dest: &Path) -> Result<()>{
    for entry in WalkDir::new(src).follow_links(true).into_iter().filter_map(std::result::Result::ok) {
        let relative_path = entry.path().strip_prefix(src).unwrap(); 

        let target_path = dest.join(relative_path); 


        if entry.path().is_dir() {
            if !target_path.exists() {
                create_directory(&target_path)?;
            }
          
        }
        else {
            copy_file(entry.path(), &target_path, src)?;
        }
    }

    Ok(())
}



pub fn generate_site(
    root_dir: &Path,
    interface: IpAddr,
    interface_port: u16,
    output_dir: Option<&Path>,
    force: bool,
    base_url: Option<&str>,
    config_file: &Path,
    mut no_port_append: bool,
) -> Result<(Site, SocketAddr, String)> {

    let mut site = Site::new(root_dir, config_file)?;
    let address = SocketAddr::new(interface, interface_port);

    //when no base url is provided, use the interface address
    let base_url = base_url.map_or_else(
        || {
            no_port_append = true; 
            address.to_string()
        },
        |xm | xm.to_string(),
    );


    if let Some(output_dir) = output_dir {
        if !force && output_dir.exists() {
            return Err(Error::msg(format!(
                "Directory '{}' already exists. Use --force to overwrite.",
                output_dir.display(),
            )));
        }
        site.set_output_path(output_dir);
    }

    site.load_files()?; 

    site.build_output_dir()?;

    Ok((site, address, base_url))
}


/// Builds the output directory for the site.
///
/// If `output_dir` is given and exists, it will be removed unless `force` is `false`.
/// If `base_url` is given, it will be used to set the base URL of the site.
/// TODO
pub fn build_output_dir(root_dir: &Path, config_file: &Path, output_dir: Option<&Path>, force: bool) -> Result<()>{
    let mut site = Site::new(root_dir, config_file)?;
    if let Some(output_dir) = output_dir {
        if !force && output_dir.exists() {
            return Err(Error::msg(format!("Output directory {} already exists", output_dir.display())));
        }

        //set the output directory
        site.set_output_path(output_dir);
    }

    // Load all content files before building the output directory
    site.load_files()?;
    
    // Build the output directory and return the result
    site.build_output_dir()?;
    
    println!("\n✅ Site built successfully!");
    println!("   Output directory: {}", site.output_path.display());
    
    Ok(())
}


// fn rebuild_done_handling(broadcaster: &Sender, res: Result<()>, reload_path: &str) {
//     match res {
//         Ok(_) => {
//             clear_serve_error();
//             broadcaster
//                 .send(format!(
//                     r#"
//                 {{
//                     "command": "reload",
//                     "path": {},
//                     "originalPath": "",
//                     "liveCSS": true,
//                     "liveImg": true,
//                     "protocol": ["http://livereload.com/protocols/official-7"]
//                 }}"#,
//                     serde_json::to_string(&reload_path).unwrap()
//                 ))
//                 .unwrap();
//         }
//         Err(e) => {
//             let msg = "Failed to build the site";

//             messages::unravel_errors(msg, &e);
//             set_serve_error(msg, e);
//         }
//     }
// }

/// Builds the output directory for the site and sends a reload message to the broadcaster
///
/// # Arguments
///
/// * `broadcaster` - A reference to the broadcaster
/// * `res` - The result of the build operation
/// * `reload_path` - The path to reload
pub fn build_output_dir_with_broadcaster(broadcaster: &Sender, res: Result<()>, reload_path: &str) {
    match res {
        Ok(_)  => {
            broadcaster.send(format!(
                r#"
                {{
                    "command": "reload",
                    "path": {},
                    "originalPath": "",
                    "liveCSS": true,
                    "liveImg": true,
                    "protocol": ["http://livereload.com/protocols/official-7"]
                }}"#,
                format!(r#""{}""#, reload_path)
            ))
            .unwrap();
        }
        Err(e) => {
            println!("Error while building the site: {}", e);
        }
    }
}