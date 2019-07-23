use crate::executors::ContextPair;
use std::path::PathBuf;

pub trait Ops {
    fn copy<S: AsRef<str>>(&self, file_name: S);
    fn copy_glob<S: AsRef<str>>(&self, pattern: S);
    fn execute<S: AsRef<str>>(&self, command: S);
}

impl Ops for ContextPair<PathBuf> {
    fn copy<S: AsRef<str>>(&self, file_name: S) {
        println!(
            "copying {} from {} to {}",
            file_name.as_ref(),
            self.0.display(),
            self.1.display()
        );
    }

    fn copy_glob<S: AsRef<str>>(&self, pattern: S) {
        println!(
            "copying files matching {} from {}",
            pattern.as_ref(),
            self.0.display()
        );
    }

    fn execute<S: AsRef<str>>(&self, command: S) {
        println!("executing '{}' in {}", command.as_ref(), self.0.display());
    }
}
