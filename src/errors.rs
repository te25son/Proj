use std::fmt::{self, Debug};
use std::io;

use crate::dependencies::Deps;

pub enum ErrorType {
    MissingDependency(Deps),
    FileNotFound(String),
    IO(io::Error),
}

impl Debug for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingDependency(deps) => write!(f, "Missing required package '{}'", deps.to_string()),
            Self::FileNotFound(file) => write!(f, "Could not find file '{}'", file),
            Self::IO(err) => write!(f, "{}", err.to_string()),
        }
    }
}

impl From<io::Error> for ErrorType {
    fn from(value: io::Error) -> Self {
        ErrorType::IO(value)
    }
}
