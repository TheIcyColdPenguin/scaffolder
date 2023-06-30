use std::{fs, path::PathBuf};

use crate::types::{Projects, Result, ScaffoldOptions};

impl ScaffoldOptions {
    pub fn get_config_file_path(&self) -> PathBuf {
        self.config.join("scaffolder.yml")
    }
    pub fn get_premade_directory_path(&self) -> PathBuf {
        self.config.join("premades")
    }
    pub fn get_template_directory_path(&self) -> PathBuf {
        self.config.join("templates")
    }

    pub fn parse_config_file(&self) -> Result<Projects> {
        let file = fs::File::open(self.get_config_file_path())?;
        Ok(serde_yaml::from_reader(file)?)
    }
}
