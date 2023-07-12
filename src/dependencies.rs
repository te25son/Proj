use std::fmt;
use std::process::{Command, Stdio};
use strum::{EnumIter, IntoEnumIterator};

use crate::errors::ErrorType;

pub const DEFAULT_DEV_DEPENDENCIES: [&str; 6] = [
    "black", "mypy", "ruff", "flake8", "flake8-type-checking", "flake8-pyproject"
];

#[derive(Debug, EnumIter)]
pub enum Deps {
    Poetry,
    Python,
    Git,
    Just,
    PreCommit,
}

impl Deps {
    pub fn check_all() -> Result<(), ErrorType> {
        for dependency in Deps::iter() {
            dependency.check()?
        }
        Ok(())
    }

    fn check(self) -> Result<(), ErrorType> {
        let status = Command::new(self.to_string())
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .arg("--version")
            .status();

        if status.is_err() {
            return Err(ErrorType::MissingDependency(self))
        }
        Ok(())
    }
}

impl fmt::Display for Deps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Deps::Poetry => write!(f, "poetry"),
            Deps::Python => write!(f, "python"),
            Deps::Git => write!(f, "git"),
            Deps::Just => write!(f, "just"),
            Deps::PreCommit => write!(f, "pre-commit"),
        }
    }
}
