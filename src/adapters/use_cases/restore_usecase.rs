use crate::adapters::context::restore_context::RestoreContext;
use crate::adapters::executor::FileSystemExecutor;
use crate::application::error::VacuumError;
use crate::application::executor;
use crate::application::usecase::UseCase;
use crate::domain::App;
use std::path::PathBuf;

pub struct RestoreUseCase {
    app_dir: PathBuf,
}

impl RestoreUseCase {
    pub fn new(app_dir: PathBuf) -> Self {
        Self { app_dir }
    }
}

impl UseCase for RestoreUseCase {
    fn run(&self, app: &App) -> Result<(), VacuumError> {
        let executor = FileSystemExecutor::new(app.name.to_string());
        executor::execute(&executor, &RestoreContext::new(self.app_dir.clone()), &app)
    }
}
