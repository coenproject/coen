use super::Coen;

impl Coen {
    pub(crate) fn command_set(&mut self) {
        // Set the variable

        println!("Encountered SET");
        println!("{:?}", self.arguments);
    }
}
