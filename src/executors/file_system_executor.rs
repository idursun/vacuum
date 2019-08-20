use super::context::Context;
use super::ops::Ops;
use crate::error::VacuumError;
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
    fn copy_file<S: AsRef<str>>(&self, file_name: S) -> Result<(), VacuumError> {
        let source = self.source.sub(file_name.as_ref());
        if !source.exists() {
            return Ok(());
        }

        if fs::create_dir_all(self.target.as_path()).is_ok() {
            let destination = self.target.sub(file_name.as_ref());
            return fs::copy(source.as_path(), destination.as_path())
                .map_err(|e| VacuumError::IoError(e))
                .map(|_| ());
        }
        Ok(())
    }

    fn copy_files<S: AsRef<str>>(&self, pattern: S) -> Result<(), VacuumError> {
        for path in self.source.search(pattern.as_ref()) {
            if path.is_dir() {
                continue;
            }
            let current = path
                .strip_prefix(self.source.as_path())
                .expect("Failed to strip prefix");
            let target = self
                .target
                .sub(current.to_str().expect("Failed to convert path to str"));
            let dest_dir = target.parent().expect("Failed to get parent directory");

            if let Ok(_) = fs::create_dir_all(&dest_dir) {
                return fs::copy(path.as_path(), target.as_path())
                    .map(|_| ())
                    .map_err(VacuumError::IoError);
            }
        }
        Ok(())
    }

    fn execute<S: AsRef<str>>(
        &self,
        command: S,
        file_name: &Option<String>,
    ) -> Result<(), VacuumError> {
        let command = command.as_ref();
        println!("executing '{}' in {}", command, self.source.display());
        let args = command.split_whitespace().collect::<Vec<_>>();
        let result = std::process::Command::new(args[0])
            .args(&args[1..])
            .output()?;
        let output = String::from_utf8(result.stdout).unwrap_or_default();

        if let Some(file_name) = file_name {
            let mut file_path = self.target.clone();
            file_path.push(file_name);
            std::fs::write(file_path.as_path(), output)?;
        }

        Ok(())
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
