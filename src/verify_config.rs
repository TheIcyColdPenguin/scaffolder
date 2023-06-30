use std::{fs, path::PathBuf};

use colored::Colorize;
use directories::ProjectDirs;

use crate::types::{Result, ScaffoldCliArgs};

pub trait VerifyConfig {
    fn verify_config(args: &mut ScaffoldCliArgs) -> Result<PathBuf> {
        let config_dir = match args.config {
            Some(ref config) => config.clone(),

            None => match ProjectDirs::from("com", "TheIcyColdPenguin", "scaffolder") {
                Some(proj_dirs) => proj_dirs.config_dir().to_path_buf(),
                None => return Err("Couldn't create or find config directory".into()),
            },
        };

        fs::create_dir_all(config_dir.join("templates"))?;
        fs::create_dir_all(config_dir.join("premades"))?;

        let config_file_path = config_dir.join("scaffolder.yml");
        let config_file = fs::OpenOptions::new()
            .create_new(true)
            .append(true)
            .open(&config_file_path);

        match config_file {
            Ok(_) => println!(
                "Created config file at '{}'",
                config_file_path.display().to_string().green().bold()
            ),
            Err(_) => {
                if args.config.is_some() {
                    println!(
                        "Found config file at '{}'",
                        config_file_path.display().to_string().green().bold()
                    )
                }
            }
        }

        Ok(config_dir)
    }
}
