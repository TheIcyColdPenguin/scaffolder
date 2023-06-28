use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::{exit, Command, Stdio},
};

use colored::Colorize;

use crate::{
    app::App,
    types::{CliCommands, CommandKind, ProjectScaffold},
};

impl App {
    pub fn process_command(&self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.options.command {
            CliCommands::List => self.list_projects(),
            CliCommands::Create { name, location } => self.create(name, location),
        }
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

    fn create(&self, name: &str, location: &Path) -> Result<(), Box<dyn std::error::Error>> {
        Self::verify_location(location)?;
        let project = dbg!(self.find_project(name)?);
        let premade_dir = self.options.get_premade_directory_path();
        let template_dir = self.options.get_template_directory_path();

        for command in project.commands {
            command.run(&premade_dir, &template_dir, location)?;
        }

        Ok(())
    }

    fn find_project(&self, name: &str) -> Result<ProjectScaffold, Box<dyn std::error::Error>> {
        let projects = self.options.parse_config_file()?;

        let Some(projects) =  projects.projects else {
            println!("{}", "No projects available".red().bold());
            exit(1);
        };

        let Some(project) =  projects
        .into_iter()
        .find(|project| project.names.contains(&name.to_owned())) else {
            println!("{}", "Specified project not found".red().bold());
            exit(1);
        };

        Ok(project)
    }

    fn verify_location(location: &Path) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(location)?;
        if let Some(_) = fs::read_dir(location)?.next() {
            println!("{}", "The specified directory is not empty".red());
            exit(1);
        }

        Ok(())
    }
}

impl CommandKind {
    pub fn run(
        &self,
        premade_location: &Path,
        template_location: &Path,
        project_location: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            CommandKind::CopyFile {
                src_file,
                dest_file,
            } => {
                let src = PathBuf::from(premade_location).join(src_file);
                let dest = PathBuf::from(project_location).join(dest_file);

                println!(
                    "Copying '{}' to '{}'.",
                    src_file.green(),
                    dest.to_string_lossy().green(),
                );

                fs::create_dir_all(dest.parent().ok_or("Couldn't find project dir")?)?;
                fs::copy(src, dest)?;

                Ok(())
            }
            CommandKind::CreateFile { file, contents } => {
                let dest = PathBuf::from(project_location).join(file);

                println!("Creating file '{}'.", dest.to_string_lossy().green(),);

                fs::create_dir_all(dest.parent().ok_or("Couldn't find project dir")?)?;
                fs::write(dest, contents)?;

                Ok(())
            }
            CommandKind::TemplateFile {
                template,
                dest_file,
                replacements,
            } => {
                let src = PathBuf::from(template_location).join(template);
                let dest = PathBuf::from(project_location).join(dest_file);

                println!(
                    "Hydrating the '{}' template to '{}'.",
                    template.green(),
                    dest.to_string_lossy().green(),
                );

                let mut contents = (fs::read_to_string(src))?;
                for (key, replacement) in replacements.iter() {
                    contents = contents.replace(&format!("${key}"), &replacement);
                }

                fs::create_dir_all(dest.parent().ok_or("Couldn't find project dir")?)?;
                fs::write(dest, contents)?;

                Ok(())
            }
            CommandKind::SingleCommand { command, args } => {
                println!("Running '{}'.", command.green());

                Command::new(command)
                    .args(args)
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()?
                    .wait()?;

                Ok(())
            }
        }
    }
}
