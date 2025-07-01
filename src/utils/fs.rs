use std::io::Write;
use std::path::{ Path, PathBuf };
use std::fs::{ create_dir, create_dir_all, File };
use anyhow::{Context, Ok, Result};


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


pub fn create_file(path: &Path, content: impl AsRef<str>) -> Result<()> {
    create_parent(path)?; 
    let mut file = 
        File::create(path).with_context(|| format!("Failed to create file {}", path.display()))?;
    
    file.write_all(content.as_ref().as_bytes()); 
    Ok(())
}


/// TODO
pub fn build_output_dir(root_dir: &Path, config_file: &Path, base_url: Option<&str>, output_dir: Option<&Path>) -> Result<()>{
    Ok(())
}