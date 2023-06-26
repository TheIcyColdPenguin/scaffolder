use std::{fs, path::PathBuf};

use crate::types::{Projects, ScaffoldOptions};

impl ScaffoldOptions {
    fn get_config_file_path(&self) -> PathBuf {
        self.config.join("scaffolder.toml")
    }

    pub fn parse_config_file(&self) -> Result<Projects, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(self.get_config_file_path())?;
        match toml::from_str(&contents) {
            Ok(projects) => Ok(projects),
            Err(err) => Err(err.message().into()),
        }
    }
}
