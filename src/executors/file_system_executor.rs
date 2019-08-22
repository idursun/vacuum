use super::context::Context;
use super::ops::Ops;
use crate::error::VacuumError;
use colored::*;
use std::fs;
use std::path::PathBuf;

#[derive(Clone)]
struct Logger<'a> {
    name: &'a str,
}

impl<'a> Logger<'a> {
    fn new(name: &'a str) -> Self {
        Self { name }
    }

    fn print<S: AsRef<str>>(&self, line: S) {
        println!("[{:<10}] {line}", self.name.green(), line = line.as_ref(),);
    }
}

#[derive(Clone)]
pub struct FileSystemExecutor<'a, C: Context> {
    source: C,
    target: C,
    logger: Logger<'a>,
}

impl<'a, C> FileSystemExecutor<'a, C>
where
    C: Context + Default,
{
    pub fn new(target_dir: C, name: &'a str) -> Self {
        Self {
            source: C::default(),
            target: target_dir,
            logger: Logger::new(name),
        }
    }
}

impl<'a> Ops for FileSystemExecutor<'a, PathBuf> {
    fn copy_file<S: AsRef<str>>(&self, file_name: S) -> Result<(), VacuumError> {
        let file_name = file_name.as_ref();
        let source = self.source.sub(file_name);
        if !source.exists() {
            return Ok(());
        }

        if fs::create_dir_all(self.target.as_path()).is_ok() {
            let destination = self.target.sub(file_name);
            self.logger
                .print(format!("{} {}", "Copy".blue(), source.display()));
            return fs::copy(source.as_path(), destination.as_path())
                .map_err(VacuumError::IoError)
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

            if fs::create_dir_all(&dest_dir).is_ok() {
                if fs::copy(path.as_path(), target.as_path()).is_ok() {
                    self.logger
                        .print(format!("{} {}", "Copy".blue(), path.display()));
                }
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
        let mut args = if cfg!(windows) {
            vec!["cmd", "/c"]
        } else {
            vec![]
        };
        args.extend(command.split_whitespace().collect::<Vec<_>>());

        let result = std::process::Command::new(args[0])
            .args(&args[1..])
            .output()?;

        if result.status.success() {
            self.logger
                .print(format!("{} {} ", "Execute".blue(), command));
            let output = String::from_utf8(result.stdout).unwrap_or_default();

            if let Some(file_name) = file_name {
                let mut file_path = self.target.clone();
                file_path.push(file_name);
                std::fs::write(file_path.as_path(), output)?;
            }

            return Ok(());
        }
        // return the error here
        Ok(())
    }
}

impl<'a> Context for FileSystemExecutor<'a, PathBuf> {
    fn home(&self) -> Self {
        Self {
            source: self.source.home(),
            target: self.target.sub("home"),
            logger: Logger {
                name: self.logger.name,
            },
        }
    }

    fn config(&self) -> Self {
        Self {
            source: self.source.config(),
            target: self.target.sub("config"),
            logger: Logger {
                name: self.logger.name,
            },
        }
    }

    fn sub<S: AsRef<str>>(&self, sub: S) -> Self {
        let sub = sub.as_ref();
        let source = self.source.sub(sub);
        let target = self.target.sub(sub);

        Self {
            source,
            target,
            logger: Logger {
                name: self.logger.name,
            },
        }
    }

    fn search(&self, pattern: &str) -> Vec<Self> {
        let mut ret = Vec::new();
        let sources = self.source.search(pattern);
        for source in sources {
            let remaining = source.strip_prefix(self.source.as_path()).unwrap();
            let target = self.target.sub(remaining.to_str().unwrap());
            ret.push(Self {
                source,
                target,
                logger: Logger {
                    name: self.logger.name,
                },
            })
        }
        ret
    }
}

impl<'a> Default for FileSystemExecutor<'a, PathBuf> {
    fn default() -> Self {
        Self {
            source: PathBuf::default(),
            target: PathBuf::default(),
            logger: Logger::new(""),
        }
    }
}
