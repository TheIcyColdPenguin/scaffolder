use std::fs;

use directories::ProjectDirs;

use crate::types::ScaffoldArgs;

pub trait VerifyConfig {
    fn verify_args(args: &mut ScaffoldArgs) -> Result<(), Box<dyn std::error::Error>> {
        let config_dir = match args.config {
            Some(ref config) => config.clone(),
            None => match ProjectDirs::from("com", "TheIcyColdPenguin", "scaffolder") {
                Some(proj_dirs) => proj_dirs.config_dir().to_path_buf(),
                None => return Err("Couldn't create or find config directory".into()),
            },
        };

        fs::create_dir_all(config_dir.join("templates"))?;
        fs::create_dir_all(config_dir.join("premade"))?;

        let config_file = fs::File::options()
            .read(true)
            .create(true)
            .append(true)
            .open(config_dir.join("scaffolder.toml"));

        dbg!(&config_file);

        args.config = Some(config_dir);

        Ok(())
    }
}
