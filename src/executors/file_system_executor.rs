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
        Self {
            source: C::default(),
            target: target_dir,
        }
    }
}

impl Ops for FileSystemExecutor<PathBuf> {
    fn copy_file<S: AsRef<str>>(&self, file_name: S) {
        let source = self.source.sub(file_name.as_ref());
        if !source.exists() {
            return;
        }

        if fs::create_dir_all(self.target.as_path()).is_ok() {
            let destination = self.target.sub(file_name.as_ref());
            fs::copy(source.as_path(), destination.as_path()).expect("failed to copy");
        }
    }

    fn copy_glob<S: AsRef<str>>(&self, pattern: S) {
        for path in self.source.search(pattern.as_ref()) {
            if path.is_dir() {
                continue;
            }
            let current = path.strip_prefix(self.source.as_path()).unwrap();
            let target = self.target.sub(current.to_str().unwrap());
            let dest_dir = target.parent().unwrap();

            if let Ok(_) = fs::create_dir_all(&dest_dir) {
                fs::copy(path.as_path(), target.as_path()).expect("failed to copy");
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
        Self {
            source: self.source.home(),
            target: self.target.sub("home"),
        }
    }

    fn config(&self) -> Self {
        Self {
            source: self.source.config(),
            target: self.target.sub("config"),
        }
    }

    fn sub<S: AsRef<str>>(&self, sub: S) -> Self {
        let source = self.source.sub(sub.as_ref());
        let target = self.target.sub(sub.as_ref());

        Self { source, target }
    }

    fn search(&self, pattern: &str) -> Vec<Self> {
        let mut ret = vec![];
        let sources = self.source.search(pattern);
        for source in sources {
            let remaining = source.strip_prefix(self.source.as_path()).unwrap();
            let target = self.target.sub(remaining.to_str().unwrap());
            ret.push(Self { source, target })
        }
        ret
    }
}

impl Default for FileSystemExecutor<PathBuf> {
    fn default() -> Self {
        Self {
            source: PathBuf::default(),
            target: PathBuf::default(),
        }
    }
}
