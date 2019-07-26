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
            "copy: {source}{separator}{file_name} -> {target}{separator}{file_name}",
            file_name = file_name.as_ref(),
            separator = std::path::MAIN_SEPARATOR,
            source = self.0.display(),
            target = self.1.display()
        );
    }

    fn copy_glob<S: AsRef<str>>(&self, pattern: S) {
        let full_pattern = format!(
            "{}{}{}",
            self.0.to_str().unwrap(),
            std::path::MAIN_SEPARATOR,
            pattern.as_ref()
        );
        for entry in glob::glob(full_pattern.as_ref()).unwrap() {
            match entry {
                Ok(path) => {
                    let current = path.strip_prefix(self.0.as_path()).unwrap();
                    let mut destination = self.1.clone();
                    destination.push(current);

                    println!(
                        "copying files matching {} from {} to {}",
                        pattern.as_ref(),
                        path.display(),
                        destination.display()
                    );
                }
                _ => (),
            }
        }
    }

    fn execute<S: AsRef<str>>(&self, command: S) {
        println!("executing '{}' in {}", command.as_ref(), self.0.display());
    }
}
