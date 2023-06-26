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
                    let mut iter = project.names.into_iter().peekable();
                    while let Some(name) = iter.next() {
                        if iter.peek().is_some() {
                            write!(stdlock, "{} | ", name.green())?;
                        } else {
                            write!(stdlock, "{} : ", name.green())?;
                        }
                    }
                    writeln!(stdlock, "{}", project.description.bold())?;
                }
            }
        };

        Ok(())
    }
}
