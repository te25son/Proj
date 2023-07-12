use include_dir::{include_dir, Dir};
use std::fs::OpenOptions;
use std::io::Write;
use strum::EnumIter;

use crate::ErrorType;
use crate::Proj;

static TEMPLATE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");


#[derive(EnumIter)]
pub enum TemplateType {
    Pyproject,
    Justfile,
    PreCommitConfig,
    PreCommitSync,
    Gitignore
}

impl TemplateType {
    pub fn copy(&self, proj: &Proj) -> Result<(), ErrorType>{
        let files = self.files();
        let source_content = files.get_source_content(proj.name())?;
        let mut target = self.options().open(
            proj.path().join(files.target())
        )?;

        target.write_all(source_content.as_bytes())?;

        Ok(())
    }

    fn files(&self) -> TemplateFiles {
        match self {
            TemplateType::Pyproject => TemplateFiles(
                String::from("pyproject_tools.toml"),
                Some(String::from("pyproject.toml"))
            ),
            TemplateType::Justfile => TemplateFiles(String::from("justfile"), None),
            TemplateType::PreCommitConfig => TemplateFiles(String::from(".pre-commit-config.yaml"), None),
            TemplateType::PreCommitSync => TemplateFiles(String::from("pre_commit_sync.json"), None),
            TemplateType::Gitignore => TemplateFiles(String::from(".gitignore"), None),
        }
    }

    fn options(&self) -> OpenOptions {
        match self {
            TemplateType::Pyproject => OpenOptions::new().write(true).append(true).to_owned(),
            _ => OpenOptions::new().write(true).create(true).to_owned()
        }
    }
}

pub struct TemplateFiles(String, Option<String>);

impl TemplateFiles {
    pub fn source(&self) -> &String {
        &self.0
    }

    pub fn target(&self) -> &String {
        self.1.as_ref().unwrap_or(self.source())
    }

    pub fn get_source_content(&self, proj_name: &String) -> Result<String, ErrorType> {
        let file_content = TEMPLATE_DIR
            .get_file(self.source())
            .map(|file| {
                file.contents_utf8().unwrap_or_default()
            });

        if let Some(content) = file_content {
            return Ok(content.replace("{{project_name}}", proj_name.as_str()))
        }
        Err(ErrorType::FileNotFound(self.source().to_string()))
    }
}
