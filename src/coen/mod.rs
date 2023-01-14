use std::{path::PathBuf, fs::File, collections::HashMap, io::{BufReader, BufRead}, error::Error};

pub(crate) struct Coen {
    project_path: PathBuf,
    info_file_path: PathBuf,
    coen_file_path: PathBuf,

    variables: HashMap<String, String>,
    current_statement: String,
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
            current_statement: String::from("")
        }
    }

    pub fn convert(&mut self) -> Result<(), Box<dyn Error>> {
        let coen_file = File::open(self.coen_file_path.clone())?;
        let reader = BufReader::new(coen_file);

        let mut pre: String;
        let mut content: String;

        for line in reader.lines() {
            let line = line?;

            match line.chars().nth(0) {
                Some(char) => {
                    if char == '!' {
                        self.execute_command(line);
                    }
                },
                None => {},
            }
        }

        Ok(())
    }

    fn execute_command(&self, command: String) {
        println!("Encountered Command: {command}");

        let arguments = command.split_whitespace();
        let arguments: Vec<&str> = arguments.collect();

        match arguments.clone().into_iter().nth(0).unwrap() {
            "!set" => {
                self.command_set();
            },
            _ => {}

        }

        println!("{arguments:?}");
    }

    fn command_set(&self) {
        println!("Encountered SET");
    }
}