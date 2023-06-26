use std::{fs, path::PathBuf};

use directories::ProjectDirs;

use crate::types::ScaffoldCliArgs;

pub trait VerifyConfig {
    fn verify_config(args: &mut ScaffoldCliArgs) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let config_dir = match args.config {
            Some(ref config) => config.clone(),

            None => match ProjectDirs::from("com", "TheIcyColdPenguin", "scaffolder") {
                Some(proj_dirs) => proj_dirs.config_dir().to_path_buf(),
                None => return Err("Couldn't create or find config directory".into()),
            },
        };

        fs::create_dir_all(config_dir.join("templates"))?;
        fs::create_dir_all(config_dir.join("premade"))?;

        let _config_file = fs::OpenOptions::new()
            .create_new(true)
            .open(config_dir.join("scaffolder.toml"));

        Ok(config_dir)
    }
}
