use crate::executors::ContextPair;
use std::{
    fs,
    path::{self, PathBuf},
};

pub trait Ops {
    fn copy<S: AsRef<str>>(&self, file_name: S);
    fn copy_glob<S: AsRef<str>>(&self, pattern: S);
    fn execute<S: AsRef<str>>(&self, command: S);
}

impl Ops for ContextPair<PathBuf> {
    fn copy<S: AsRef<str>>(&self, file_name: S) {
        let mut source = self.0.clone();
        source.push(file_name.as_ref());
        if !source.exists() {
            return;
        }

        let mut destination = self.1.clone();
        if let Ok(_) = fs::create_dir_all(destination.as_path()) {
            destination.push(file_name.as_ref());
            fs::copy(source.as_path(), destination.as_path()).expect("failed to copy");
        }
    }

    fn copy_glob<S: AsRef<str>>(&self, pattern: S) {
        let full_pattern = format!(
            "{}{}{}",
            self.0.to_str().unwrap(),
            path::MAIN_SEPARATOR,
            pattern.as_ref()
        );

        for entry in glob::glob(full_pattern.as_ref()).unwrap() {
            match entry {
                Ok(path_buf) => {
                    if path_buf.is_dir() {
                        continue;
                    }
                    let current = path_buf.strip_prefix(self.0.as_path()).unwrap();
                    let mut destination = self.1.clone();
                    destination.push(current);
                    let dest_dir = destination.parent().unwrap();

                    if let Ok(_) = fs::create_dir_all(&dest_dir) {
                        fs::copy(path_buf.as_path(), destination.as_path())
                            .expect("failed to copy");
                    }
                }
                _ => (),
            }
        }
    }

    fn execute<S: AsRef<str>>(&self, command: S) {
        println!("executing '{}' in {}", command.as_ref(), self.0.display());
    }
}
