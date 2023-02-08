use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use super::CoenBuilder;

static COMAND_IDENTIFIER: char = '!';

impl CoenBuilder {
    pub fn build(&mut self) -> Result<(), Box<dyn Error>> {
        self.build_content()?;

        Ok(())
    }

    pub(crate) fn build_content(&mut self) -> Result<(), Box<dyn Error>> {
        if self
            .conversion_files
            .contains(&self.current_conversion_file)
        {
            return Err("Program contains import loops.".into());
        }

        self.conversion_files
            .insert(self.current_conversion_file.clone());

        let file = File::open(&self.current_conversion_file)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            self.current_statement = line?;

            match self.current_statement.chars().nth(0) {
                Some(ch) => {
                    if ch == COMAND_IDENTIFIER {
                        self.handle_command()?;
                    } else {
                        self.handle_statement();
                    }
                }
                None => {}
            }
        }

        self.conversion_files.remove(&self.current_conversion_file);

        Ok(())
    }

    pub(crate) fn handle_command(&mut self) -> Result<(), Box<dyn Error>> {
        let mut modified_statement = self.current_statement.chars();
        modified_statement.next();
        let modified_statement = modified_statement.as_str();
        let elements: Vec<&str> = modified_statement.split_whitespace().collect();

        match elements[0] {
            "import" => {
                self.command_import()?;
            }
            _ => {
                println!("Invalid command: {}", elements[0]);
                self.command_default()?;
            }
        }

        Ok(())
    }

    pub(crate) fn handle_statement(&self) {
        println!("{}", self.current_statement);
    }

    pub(crate) fn get_joined_elements(elements: &Vec<&str>, start: usize) -> String {
        let mut result_vec = Vec::new();

        for i in start..elements.len() {
            result_vec.push(elements[i]);
        }

        let result = result_vec.join(" ");

        result
    }
}
