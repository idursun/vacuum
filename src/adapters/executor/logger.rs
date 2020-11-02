use colored::*;

#[derive(Clone)]
pub struct Logger {
    pub name: String,
}

impl Logger {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn print<S: AsRef<str>>(&self, line: S) {
        println!("[{:<10}] {line}", self.name.green(), line = line.as_ref(),);
    }
}
