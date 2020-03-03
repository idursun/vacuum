use crate::application::context::Context;
use crate::application::error::VacuumError;

pub trait Handler {
    type Context: Context;
    fn handle_file<S: AsRef<str>>(
        &self,
        ctx: &Self::Context,
        file_name: S,
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
