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

use crate::{
    types::{ScaffoldCliArgs, ScaffoldOptions},
    verify_config::VerifyConfig,
};

#[derive(Debug)]
pub struct App {
    pub options: ScaffoldOptions,
}

impl App {
    pub fn new(mut args: ScaffoldCliArgs) -> Result<App, Box<dyn std::error::Error>> {
        let config = Self::verify_config(&mut args)?;

        Ok(App {
            options: ScaffoldOptions {
                config,
                command: args.command,
            },
        })
    }
}

impl VerifyConfig for App {}
