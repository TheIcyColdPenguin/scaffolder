use std::{fs, path::PathBuf};

use crate::types::{Projects, ScaffoldOptions};

impl ScaffoldOptions {
    pub fn get_config_file_path(&self) -> PathBuf {
        self.config.join("scaffolder.toml")
    }
    pub fn get_premade_directory_path(&self) -> PathBuf {
        self.config.join("premades")
    }
    pub fn get_template_directory_path(&self) -> PathBuf {
        self.config.join("templates")
    }

    pub fn parse_config_file(&self) -> Result<Projects, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(self.get_config_file_path())?;
        match toml::from_str(&contents) {
            Ok(projects) => Ok(projects),
            Err(err) => Err(err.message().into()),
        }
    }
}
