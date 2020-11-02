use crate::adapters::context::store_context::StoreContext;
use crate::adapters::executor::FileSystemExecutor;
use crate::application::error::VacuumError;
use crate::application::executor;
use crate::application::usecase::UseCase;
use crate::domain::App;
use std::path::PathBuf;

pub struct StoreUseCase {
    app_dir: PathBuf,
}

impl StoreUseCase {
    pub fn new(app_dir: PathBuf) -> Self {
        Self { app_dir }
    }
}

impl UseCase for StoreUseCase {
    fn run(&self, app: &App) -> Result<(), VacuumError> {
        let executor = FileSystemExecutor::new(app.name.to_string());
        executor::execute(&executor, &StoreContext::new(self.app_dir.clone()), &app)
    }
}
