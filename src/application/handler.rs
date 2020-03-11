use crate::application::context::Context;
use crate::application::error::VacuumError;
use crate::domain::DependencyCheck;

pub trait Handler {
    type Context: Context;
    fn handle_file<S: AsRef<str>>(
        &self,
        ctx: &Self::Context,
        file_name: S,
        dependency_checks: &Option<Vec<DependencyCheck>>,
    ) -> Result<(), VacuumError>;
    fn handle_files<S: AsRef<str>>(
        &self,
        ctx: &Self::Context,
        pattern: S,
    ) -> Result<(), VacuumError>;
    fn handle_execute<S: AsRef<str>>(
        &self,
        ctx: &Self::Context,
        command: S,
        file_name: &Option<String>,
    ) -> Result<(), VacuumError>;
}
