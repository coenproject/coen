use std::{error::Error, path::PathBuf};

mod coen;
mod config;

fn main() -> Result<(), Box<dyn Error>> {
    let cwd = std::env::current_dir()?;

    let mut project_path = PathBuf::new();
    project_path.push(cwd);
    project_path.push(config::PROJECT_PATH);

    let mut coen_instance = coen::Coen::new(project_path);

    coen_instance.convert()?;

    let output = coen_instance.get_output();
    println!("OUTPUT:");
    println!("{output}");

    Ok(())
}
