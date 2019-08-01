use super::context::Context;
use super::ops::Ops;
use std::fs;
use std::path::PathBuf;

#[derive(Clone)]
pub struct FileSystemExecutor<C: Context> {
    source: C,
    target: C,
}

impl<C> FileSystemExecutor<C>
where
    C: Context + Default,
{
    pub fn new(target_dir: C) -> Self {
        FileSystemExecutor {
            source: C::default(),
            target: target_dir,
        }
    }
}

impl Ops for FileSystemExecutor<PathBuf> {
    fn copy<S: AsRef<str>>(&self, file_name: S) {
        let mut source = self.source.clone();
        source.push(file_name.as_ref());
        if !source.exists() {
            return;
        }

        let mut destination = self.target.clone();
        if fs::create_dir_all(destination.as_path()).is_ok() {
            destination.push(file_name.as_ref());
            fs::copy(source.as_path(), destination.as_path()).expect("failed to copy");
        }
    }

    fn copy_glob<S: AsRef<str>>(&self, pattern: S) {
        let full_pattern = format!(
            "{}{}{}",
            self.source.to_str().unwrap(),
            std::path::MAIN_SEPARATOR,
            pattern.as_ref()
        );
        for entry in glob::glob(full_pattern.as_ref()).unwrap() {
            if let Ok(path) = entry {
                let current = path.strip_prefix(self.source.as_path()).unwrap();
                let mut destination = self.target.clone();
                destination.push(current);

                println!(
                    "copying files matching {} from {} to {}",
                    pattern.as_ref(),
                    path.display(),
                    destination.display()
                );
            }
        }
    }

    fn execute<S: AsRef<str>>(&self, command: S) {
        println!(
            "executing '{}' in {}",
            command.as_ref(),
            self.source.display()
        );
    }
}

impl Context for FileSystemExecutor<PathBuf> {
    fn home(&self) -> Self {
        let mut destination = self.target.clone();
        destination.push("home");
        FileSystemExecutor {
            source: self.source.home(),
            target: destination,
        }
    }

    fn config(&self) -> Self {
        let mut destination = self.target.clone();
        destination.push("config");
        FileSystemExecutor {
            source: self.source.config(),
            target: destination,
        }
    }

    fn sub<S: AsRef<str>>(&self, sub: S) -> Self {
        let FileSystemExecutor {
            mut source,
            mut target,
        } = self.clone();
        source.push(sub.as_ref());
        target.push(sub.as_ref());

        FileSystemExecutor { source, target }
    }

    fn search(&self, pattern: &str) -> Vec<Self> {
        let mut ret = vec![];
        let sources = self.source.search(pattern);
        for source in sources {
            let remaining = source.strip_prefix(self.source.as_path()).unwrap();
            let mut new_destination = self.target.clone();
            new_destination.push(remaining);
            ret.push(FileSystemExecutor {
                source,
                target: new_destination,
            })
        }
        ret
    }
}

impl Default for FileSystemExecutor<PathBuf> {
    fn default() -> Self {
        FileSystemExecutor {
            source: PathBuf::default(),
            target: PathBuf::default(),
        }
    }
}
