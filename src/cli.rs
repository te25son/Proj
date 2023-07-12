use clap::{Parser, Subcommand, Args, ArgAction};
use std::{process::Command, path::PathBuf};
use std::fs;
use strum::IntoEnumIterator;

use crate::dependencies::{Deps, DEFAULT_DEV_DEPENDENCIES};
use crate::errors::ErrorType;
use crate::templates::TemplateType;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Proj {
    #[command(subcommand)]
    commands: Commands,
    #[arg(short, long, action = ArgAction::SetTrue)]
    vscode: bool
} 

#[derive(Subcommand, Debug)]
enum Commands {
    New(NewArgs)
}

#[derive(Args, Debug)]
struct NewArgs {
    #[arg(long, short)]
    name: NameString
}

#[derive(Debug, Clone)]
struct NameString(String);

impl Proj {
    pub fn name(&self) -> &String {
        match &self.commands {
            Commands::New(args) => &args.name.value(),
        }
    }

    pub fn path(&self) -> PathBuf {
        fs::canonicalize(format!("./{}", self.name())).unwrap()
    }

    pub fn create(&self) -> Result<(), ErrorType>{
        // Create new poetry project
        Command::new(Deps::Poetry.to_string())
            .args(["new", self.name()])
            .status()?;

        // Add dev dependencies
        Command::new(Deps::Poetry.to_string())
            .current_dir(self.path())
            .args(["add", "--group", "dev"])
            .args(DEFAULT_DEV_DEPENDENCIES)
            .status()?;

        // Initialize new project as git repo
        Command::new(Deps::Git.to_string())
            .current_dir(self.path())
            .arg("init")
            .status()?;

        // Copy template files to new project
        self.copy_templates()?;

        Ok(())
    }

    fn copy_templates(&self) -> Result<(), ErrorType> {
        for template in TemplateType::iter() {
            template.copy(self)?
        }
        Ok(())
    }
} 

impl NameString {
    fn value(&self) -> &String {
        &self.0
    }
}

impl From<String> for NameString {
    fn from(value: String) -> Self {
        NameString(value.replace("-", "_"))
    }
}
