use std::{collections::HashSet, error::Error, fs, path::PathBuf};

use indexmap::IndexMap;

#[derive(Debug)]
pub struct CoenBuilder {
    target: Option<PathBuf>,
    silent: bool,

    content: String,
    variables: IndexMap<String, String>,
    replacements: IndexMap<String, String>,

    current_conversion_file: PathBuf,
    conversion_files: HashSet<PathBuf>,

    current_statement: String,
}

impl CoenBuilder {
    pub fn new(root_path: PathBuf, silent: bool) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            target: None,
            silent,
            content: String::new(),
            variables: IndexMap::new(),
            replacements: IndexMap::new(),
            current_conversion_file: root_path,
            conversion_files: HashSet::new(),
            current_statement: String::new(),
        })
    }

    pub fn write(&self, target_override: Option<PathBuf>) -> Result<(), Box<dyn Error>> {
        let target = match target_override {
            Some(target) => target,
            None => {
                let mut target = PathBuf::from(self.current_conversion_file.parent().unwrap());
                target.push(self.target.clone().ok_or("No target path specified.")?);

                target
            }
        };

        self.log(&format!("Writing to {}", target.display()))?;

        fs::create_dir_all(target.parent().unwrap())?;
        fs::write(target, self.content.clone())?;

        Ok(())
    }
}

mod build;
mod commands;
