use crate::adapters::context::deps_context::TargetDirectoryContext;
use crate::application::context::Context;
use crate::application::error::VacuumError;
use crate::application::executor;
use crate::application::usecase::UseCase;
use crate::application::Handler;
use crate::domain::{App, DependencyCheck};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub struct DepsUseCase {
    app_dir: PathBuf,
}

impl DepsUseCase {
    pub fn new(app_dir: PathBuf) -> Self {
        Self { app_dir }
    }
}

struct DependencyAnalyzer;

impl DependencyAnalyzer {
    fn new() -> Self {
        DependencyAnalyzer {}
    }

    fn analyze(
        &self,
        file_path: PathBuf,
        dependency_checks: &Vec<DependencyCheck>,
    ) -> Result<(), VacuumError> {
        for check in dependency_checks {
            dbg!(&check);
            match check {
                DependencyCheck::Exists(rule) => {
                    if file_path.exists() {
                        println!("Dependency: {}", rule);
                    }
                }
                DependencyCheck::Contains(content, rule) => {
                    if file_path.exists() {
                        let mut file = File::open(file_path.as_path())?;
                        let mut contents = String::new();
                        file.read_to_string(&mut contents)?;
                        if contents.contains(content) {
                            println!("Dependency: {}", rule);
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

impl Handler for DependencyAnalyzer {
    type Context = TargetDirectoryContext;

    fn handle_file<S: AsRef<str>>(
        &self,
        ctx: &Self::Context,
        file_name: S,
        dependency_checks: &Option<Vec<DependencyCheck>>,
    ) -> Result<(), VacuumError> {
        let mut file_path = ctx.current();
        file_path.push(file_name.as_ref());
        if dependency_checks.is_some() {
            self.analyze(file_path.clone(), dependency_checks.as_ref().unwrap())?;
        }
        Ok(())
    }

    fn handle_files<S: AsRef<str>>(&self, _: &Self::Context, _: S) -> Result<(), VacuumError> {
        Ok(())
    }

    fn handle_execute<S: AsRef<str>>(
        &self,
        _: &Self::Context,
        _: S,
        _: &Option<String>,
    ) -> Result<(), VacuumError> {
        Ok(())
    }
}

impl UseCase for DepsUseCase {
    fn run(&self, app: &App) -> Result<(), VacuumError> {
        let executor = DependencyAnalyzer::new();
        executor::execute(
            &executor,
            &TargetDirectoryContext::new(self.app_dir.clone()),
            &app,
        )
    }
}
