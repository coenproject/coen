use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

mod commands;
mod paths;

pub(crate) struct Coen {
    project_path: PathBuf,

    variables: HashMap<String, String>,

    current_statement: String,
    arguments: Vec<String>,

    output: String,
}

impl Coen {
    pub fn new(project_path: PathBuf) -> Self {
        Self {
            project_path,
            variables: HashMap::new(),
            current_statement: String::from(""),
            arguments: Vec::new(),
            output: String::from(""),
        }
    }

    pub fn convert(&mut self) -> Result<(), Box<dyn Error>> {
        let coen_file = File::open(self.get_coen_file_path().clone())?;
        let reader = BufReader::new(coen_file);

        for line in reader.lines() {
            self.current_statement = line?;

            match self.current_statement.chars().nth(0) {
                Some(char) => {
                    if char == '!' {
                        self.execute_command();
                    } else {
                        self.command_write();
                    }
                }
                None => self.command_newline(),
            }
        }

        Ok(())
    }

    fn execute_command(&mut self) {
        let arguments = self.current_statement.split_whitespace();
        let arguments: Vec<&str> = arguments.collect();

        for param in arguments.iter().skip(1) {
            self.arguments.push(param.to_string());
        }

        match arguments.first().unwrap() {
            &"!set" => self.command_set(),
            &"!heading" => self.command_heading(),
            _ => self.command_unknown(),
        }

        self.arguments.clear();
    }

    pub fn get_output(&self) -> String {
        self.output.clone()
    }
}
