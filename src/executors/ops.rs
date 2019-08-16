use crate::error::VacuumError;

pub trait Ops {
    fn copy_file<S: AsRef<str>>(&self, file_name: S) -> Result<(), VacuumError>;
    fn copy_files<S: AsRef<str>>(&self, pattern: S) -> Result<(), VacuumError>;
    fn execute<S: AsRef<str>>(&self, command: S) -> Result<(), VacuumError>;
}
