use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

mod commands;

pub(crate) struct Coen {
    project_path: PathBuf,
    info_file_path: PathBuf,
    coen_file_path: PathBuf,

    variables: HashMap<String, String>,

    current_statement: String,
    arguments: Vec<String>,

    output: String,
}

impl Coen {
    pub fn new(project_path: PathBuf) -> Self {
        let mut info_file_path = PathBuf::new();
        info_file_path.push(project_path.clone());
        info_file_path.push("info.txt");

        let mut coen_file_path = PathBuf::new();
        coen_file_path.push(project_path.clone());
        coen_file_path.push("main.coen");

        Self {
            project_path,
            info_file_path,
            coen_file_path,
            variables: HashMap::new(),
            current_statement: String::from(""),
            arguments: Vec::new(),
            output: String::from(""),
        }
    }

    pub fn convert(&mut self) -> Result<(), Box<dyn Error>> {
        let coen_file = File::open(self.coen_file_path.clone())?;
        let reader = BufReader::new(coen_file);

        for line in reader.lines() {
            self.current_statement = line?;

            match self.current_statement.chars().nth(0) {
                Some(char) => {
                    if char == '!' {
                        self.execute_command();
                    }
                }
                None => {}
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
            &"!set" => {
                self.command_set();
            }
            _ => {}
        }

        self.arguments.clear();
    }

    pub fn get_output(&self) -> String {
        self.output.clone()
    }
}
