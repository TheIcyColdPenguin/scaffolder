/*
scaffolder config folder
├───scaffolder.toml
├───templates
│   │
│   ├───index.html
│   └───package.json
└───premade
    ├───project-name
    │   └───lib.rs
    └───project-name
        ├───shader.vert
        └───shader.frag

*/

use crate::{types::ScaffoldArgs, verify_config::VerifyConfig};

#[derive(Debug)]
pub struct App {
    args: ScaffoldArgs,
}

impl App {
    pub fn new(mut args: ScaffoldArgs) -> Result<App, Box<dyn std::error::Error>> {
        Self::verify_args(&mut args)?;

        Ok(App { args })
    }
}

impl VerifyConfig for App {}
