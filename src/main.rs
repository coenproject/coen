use std::{error::Error, path::PathBuf};

mod config;
mod coen;

fn main() -> Result<(), Box<dyn Error>> {
    let cwd = std::env::current_dir();

    let mut project_path = PathBuf::new();
    project_path.push(cwd.unwrap());
    project_path.push(config::PROJECT_PATH);

    let mut coen_instance = coen::Coen::new(project_path);

    coen_instance.convert()?;

    Ok(())
}