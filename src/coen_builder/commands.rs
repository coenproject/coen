use std::{error::Error, path::PathBuf};

use super::CoenBuilder;

impl CoenBuilder {
    pub(crate) fn command_import(&mut self) -> Result<(), Box<dyn Error>> {
        let elements: Vec<&str> = self.current_statement.split_whitespace().collect();

        let mut next_file_path = self.current_conversion_file.parent().unwrap().to_path_buf();

        let next_file_name = Self::get_joined_elements(&elements, 1);

        next_file_path.push(next_file_name);

        self.log(&format!("Importing File: {}", next_file_path.display()))?;

        self.current_conversion_file = PathBuf::from(next_file_path);

        self.build_content()?;

        Ok(())
    }

    pub(crate) fn command_set(&mut self) -> Result<(), Box<dyn Error>> {
        let elements: Vec<&str> = self.current_statement.split_whitespace().collect();

        let key = elements[1].to_string();
        let value = Self::get_joined_elements(&elements, 2);

        self.log(&format!("Adding Variable: {key} - {value}"))?;

        self.variables.insert(key, value);

        Ok(())
    }

    pub(crate) fn command_def(&mut self) -> Result<(), Box<dyn Error>> {
        let elements: Vec<&str> = self.current_statement.split_whitespace().collect();

        let key = elements[1].to_string();
        let value = Self::get_joined_elements(&elements, 2);

        self.log(&format!("Adding Definition: {key} - {value}"))?;

        self.replacements.insert(key, value.replace(r"\n", "\n"));

        Ok(())
    }

    pub(crate) fn command_default(&mut self) -> Result<(), Box<dyn Error>> {
        let command: Vec<&str> = self.current_statement.split_whitespace().collect();

        self.log(&format!("Invalid Command: {}", command[0]))?;

        Ok(())
    }

    pub(crate) fn log(&self, message: &str) -> Result<(), Box<dyn Error>> {
        if !self.silent {
            println!("{}", message);
        }

        Ok(())
    }
}
