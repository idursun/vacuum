use super::logger::Logger;
use crate::application::context::Context;
use crate::application::error::VacuumError;
use crate::application::handler::Handler;
use crate::domain::DependencyCheck;
use colored::*;
use std::fs;
use std::marker::PhantomData;
use std::path::PathBuf;

#[derive(Clone)]
pub struct FileSystemExecutor<C> {
    logger: Logger,
    _phantom: PhantomData<C>,
}

impl<C> FileSystemExecutor<C> {
    pub fn new(name: String) -> Self {
        Self {
            logger: Logger::new(name),
            _phantom: Default::default(),
        }
    }
}

impl<C> Handler for FileSystemExecutor<C>
where
    C: Context<Current = (PathBuf, PathBuf)>,
{
    type Context = C;

    fn handle_file<S: AsRef<str>>(
        &self,
        ctx: &Self::Context,
        file_name: S,
        _: &Option<Vec<DependencyCheck>>,
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

    fn handle_files<S: AsRef<str>>(
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

            fs::create_dir_all(&dest_dir)
                .and_then(|_| fs::copy(source.as_path(), target.as_path()))
                .and_then(|_| {
                    self.logger
                        .print(format!("{} {}", "Copy".blue(), source.display()));
                    Ok(())
                })?;
        }
        Ok(())
    }

    fn handle_execute<S: AsRef<str>>(
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
                let (_, mut target) = ctx.current();
                target.push(file_name);
                std::fs::write(target.as_path(), output)?;
            }

            return Ok(());
        }
        // return the error here
        Ok(())
    }
}
