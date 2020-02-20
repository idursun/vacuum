use colored::*;

#[derive(Clone)]
pub struct Logger<'a> {
    pub name: &'a str,
}

impl<'a> Logger<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }

    pub fn print<S: AsRef<str>>(&self, line: S) {
        println!("[{:<10}] {line}", self.name.green(), line = line.as_ref(),);
    }
}
