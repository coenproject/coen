mod config;

use std::collections::{HashMap, HashSet};

pub struct Coen {
    content: String,
    variables: HashMap<String, String>,
    commands: HashMap<String, String>,
    conversion_files: HashSet<String>,
}

mod create;
