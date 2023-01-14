use super::Coen;

impl Coen {
    pub(crate) fn command_set(&mut self) {
        // Set the variable

        println!("Encountered SET");
        println!("{:?}", self.arguments);
    }

    pub(crate) fn command_heading(&mut self) {
        self.output.push_str("\\section{");
        let mut is_first = true;

        for word in self.arguments.clone().into_iter() {
            if !is_first {
                self.output.push_str(" ");
                is_first = false;
            }

            self.output.push_str(word.as_str());
        }

        self.output.push_str("}");
        self.command_newline();
    }

    pub(crate) fn command_write(&mut self) {
        self.output.push_str(&self.current_statement);
        self.command_newline();
    }

    pub(crate) fn command_newline(&mut self) {
        self.output.push_str("\n");
    }

    pub(crate) fn command_unknown(&mut self) {
        self.output.push_str("UNKNOWN\n");
    }
}
