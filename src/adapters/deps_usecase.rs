use crate::adapters::context::deps_context::TargetDirectoryContext;
use crate::application::context::Context;
use crate::application::error::VacuumError;
use crate::application::executor;
use crate::application::usecase::UseCase;
use crate::application::Handler;
use crate::domain::App;
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

struct FileReaderExecutor<'a> {
    app: &'a App,
}

impl<'a> FileReaderExecutor<'a> {
    fn new(app: &'a App) -> Self {
        FileReaderExecutor { app }
    }
}

impl<'a> Handler for FileReaderExecutor<'a> {
    type Context = TargetDirectoryContext;

    fn handle_file<S: AsRef<str>>(
        &self,
        ctx: &Self::Context,
        file_name: S,
    ) -> Result<(), VacuumError> {
        let mut file_path = ctx.current();
        file_path.push(file_name.as_ref());
        if file_path.exists() {
            if file_name.as_ref() == "alacritty.yml" {
                println!("Alacritty is required");
            }
            let mut contents = String::new();
            let mut file = std::fs::File::open(file_path)?;
            file.read_to_string(&mut contents)?;
            if contents.contains("Plug") {
                println!("VimPlug is required");
            }
            //println!("{}", contents);
        }
        Ok(())
    }

    fn handle_files<S: AsRef<str>>(
        &self,
        ctx: &Self::Context,
        pattern: S,
    ) -> Result<(), VacuumError> {
        Ok(())
    }

    fn handle_execute<S: AsRef<str>>(
        &self,
        ctx: &Self::Context,
        command: S,
        file_name: &Option<String>,
    ) -> Result<(), VacuumError> {
        Ok(())
    }
}

impl UseCase for DepsUseCase {
    fn run(&self, app: &App) -> Result<(), VacuumError> {
        let executor = FileReaderExecutor::new(app);
        executor::execute(
            &executor,
            &TargetDirectoryContext::new(self.app_dir.clone()),
            &app,
        )
    }
}
