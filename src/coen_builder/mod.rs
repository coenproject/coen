use std::{collections::HashSet, error::Error, fs, path::PathBuf};

use indexmap::IndexMap;

use crate::args::CoenArgs;

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
    pub fn new(args: &CoenArgs) -> Result<Self, Box<dyn Error>> {
        let root_path = Self::get_root_path(&args.root)?;

        let target = match args.target.clone() {
            Some(p) => Some(fs::canonicalize(p)?),
            None => None,
        };

        Ok(Self {
            target,
            silent: args.silent,
            content: String::new(),
            variables: IndexMap::new(),
            replacements: IndexMap::new(),
            current_conversion_file: root_path,
            conversion_files: HashSet::new(),
            current_statement: String::new(),
        })
    }

    fn get_root_path(path: &PathBuf) -> Result<PathBuf, Box<dyn Error>> {
        let mut root_path = fs::canonicalize(&path)?;

        if root_path.is_dir() {
            root_path.push("root.coen");

            while !root_path.exists() {
                root_path.pop();

                if !root_path.pop() {
                    return Err("Cannot locate root file.".into());
                }

                root_path.push("root.coen");
            }
        }

        Ok(root_path)
    }

    pub fn get_content(&self) -> String {
        self.content.clone()
    }

    pub fn get_target(&self) -> Option<PathBuf> {
        self.target.clone()
    }
}

mod build;
mod commands;
