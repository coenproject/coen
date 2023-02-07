use std::{
    collections::{HashMap, HashSet},
    error::Error,
    path::PathBuf,
};

#[derive(Debug)]
pub struct CoenBuilder {
    root_path: PathBuf,

    content: String,
    variables: HashMap<String, String>,
    replacements: HashMap<String, String>,
    conversion_files: HashSet<String>,
}

impl CoenBuilder {
    pub fn new(mut root_path: PathBuf) -> Result<Self, Box<dyn Error>> {
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

        Ok(Self {
            root_path: root_path.clone(),
            content: String::new(),
            variables: HashMap::new(),
            replacements: HashMap::new(),
            conversion_files: HashSet::new(),
        })
    }

    pub fn build(&self) -> Result<(), Box<dyn Error>> {
        //

        Ok(())
    }
}
