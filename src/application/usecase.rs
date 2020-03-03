use crate::application::error::VacuumError;
use crate::domain::App;

pub trait UseCase {
    fn run(&self, app: &App) -> Result<(), VacuumError>;
}
