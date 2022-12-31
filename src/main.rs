use std::{path::PathBuf, fs::{self, File}, collections::HashMap, io::{BufReader, BufRead}, error::Error};

mod config;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize paths

    let cwd = std::env::current_dir();

    let mut project_path = PathBuf::new();
    project_path.push(cwd.unwrap());
    project_path.push(config::PROJECT_PATH);

    let mut info_file_path = PathBuf::new();
    info_file_path.push(project_path.clone());
    info_file_path.push("info.txt");

    let mut coen_file_path = PathBuf::new();
    coen_file_path.push(project_path.clone());
    coen_file_path.push("main.coen");
    
    // Initialize Stash. Store variables here.

    let mut stash = HashMap::<String, String>::new();
    stash.insert(String::from("TODAY"), String::from("Dec 31, 2022"));

    // Try reading the Coen file one line at a time

    let coen_file = File::open(coen_file_path)?;
    let reader = BufReader::new(coen_file);

    for line in reader.lines() {
        let line = line?;

        println!("{}", line);

        match line.chars().nth(0) {
            Some(char) => {
                if char == '!' {
                    println!("Encountered Command");

                    todo!();
                }
            },
            None => {},
        }
    }

    Ok(())
}