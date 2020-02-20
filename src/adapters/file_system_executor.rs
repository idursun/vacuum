use super::logger::Logger;
use super::ops::Ops;
use crate::application::context::Context;
use crate::application::ops::Ops;
use crate::context::Context;
use crate::error::VacuumError;
use colored::*;
use std::fs;
use std::marker::PhantomData;
use std::path::PathBuf;

#[derive(Clone)]
pub struct FileSystemExecutor<'a, C> {
    logger: Logger<'a>,
    _phantom: PhantomData<C>,
}

impl<'a, C> FileSystemExecutor<'a, C> {
    pub fn new(name: &'a str) -> Self {
        Self {
            logger: Logger::new(name),
            _phantom: Default::default(),
        }
    }
}

impl<'a, C> Ops for FileSystemExecutor<'a, C>
where
    C: Context<Current = (PathBuf, PathBuf)>,
{
    type Context = C;

    fn copy_file<S: AsRef<str>>(
        &self,
        ctx: &Self::Context,
        file_name: S,
    ) -> Result<(), VacuumError> {
        let (source, target) = ctx.sub(file_name.as_ref()).current();
        if !source.exists() {
            return Ok(());
        }

        let parent = target.parent().unwrap();
        if fs::create_dir_all(parent).is_ok() {
            self.logger
                .print(format!("{} {}", "Copy".blue(), source.display()));
            return fs::copy(source.as_path(), target.as_path())
                .map_err(VacuumError::IoError)
                .map(|_| ());
        }
        Ok(())
    }

    fn copy_files<S: AsRef<str>>(
        &self,
        ctx: &Self::Context,
        pattern: S,
    ) -> Result<(), VacuumError> {
        for found in ctx.search(pattern.as_ref()) {
            let (source, target) = found.current();
            if source.is_dir() {
                continue;
            }
            let dest_dir = target.parent().expect("Failed to get parent directory");

            if fs::create_dir_all(&dest_dir).is_ok() {
                if fs::copy(source.as_path(), target.as_path()).is_ok() {
                    self.logger
                        .print(format!("{} {}", "Copy".blue(), source.display()));
                }
            }
        }
        Ok(())
    }

    fn execute<S: AsRef<str>>(
        &self,
        ctx: &Self::Context,
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
                let (_, target) = ctx.current();
                let mut file_path = target.clone();
                file_path.push(file_name);
                std::fs::write(file_path.as_path(), output)?;
            }

            return Ok(());
        }
        // return the error here
        Ok(())
    }
}
