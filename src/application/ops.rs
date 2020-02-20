use crate::application::context::Context;
use crate::error::VacuumError;

pub trait Ops {
    type Context: Context;
    fn copy_file<S: AsRef<str>>(
        &self,
        ctx: &Self::Context,
        file_name: S,
    ) -> Result<(), VacuumError>;
    fn copy_files<S: AsRef<str>>(&self, ctx: &Self::Context, pattern: S)
        -> Result<(), VacuumError>;
    fn execute<S: AsRef<str>>(
        &self,
        ctx: &Self::Context,
        command: S,
        file_name: &Option<String>,
    ) -> Result<(), VacuumError>;
}
