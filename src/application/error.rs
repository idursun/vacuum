use std::error::Error;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum VacuumError {
    IoError(std::io::Error),
    ParseError(pom::Error),
}

impl Display for VacuumError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            VacuumError::ParseError(e) => write!(f, "{:?}", e),
            VacuumError::IoError(e) => write!(f, "IO Error: {}", e),
        }
    }
}

impl Error for VacuumError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl From<std::io::Error> for VacuumError {
    fn from(e: std::io::Error) -> Self {
        VacuumError::IoError(e)
    }
}
