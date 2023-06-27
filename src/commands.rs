use std::io::{self, Write};

use colored::Colorize;

use crate::{app::App, types::CliCommands};

impl App {
    pub fn process_command(&self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.options.command {
            CliCommands::List => {
                self.list_projects()?;
            }
            CliCommands::Create { name, location } => {}
        };
        Ok(())
    }

    fn list_projects(&self) -> Result<(), Box<dyn std::error::Error>> {
        let projects = self.options.parse_config_file()?;

        match projects.projects {
            None => {
                println!("{}", "No projects available".red().bold())
            }
            Some(projects) => {
                let mut stdlock = io::stdout().lock();

                writeln!(stdlock, "All projects -")?;

                for project in projects {
                    let green_names: Vec<_> = project
                        .names
                        .iter()
                        .map(|name| name.green().to_string())
                        .collect();

                    writeln!(
                        stdlock,
                        "{}",
                        format!(
                            "{all: <width$} : {desc: <30}",
                            all = green_names.join(" | "),
                            desc = project.description.bold(),
                            width = if green_names.len() == 1 { 31 } else { 40 }
                        )
                    )?;
                }
            }
        };

        Ok(())
    }
}
