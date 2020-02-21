use crate::application::error::VacuumError;
use crate::domain::App;

pub trait VacuumFileParser {
    fn parse(input: String) -> Result<App, VacuumError>;
}
