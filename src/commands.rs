use libc::system;
use std::{
    ffi::CString,
    fs,
    io::{self, Write},
    path::Path,
    process::{exit, Command, Stdio},
};

use colored::Colorize;

use crate::{
    app::App,
    types::{CliCommands, ProjectScaffold, Result, Step},
};

impl App {
    pub fn process_command(&self) -> Result<()> {
        match &self.options.command {
            CliCommands::List => self.list_projects(),
            CliCommands::Create { name, location } => self.create(name, location),
        }
    }

    fn list_projects(&self) -> Result<()> {
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

    fn create(&self, name: &str, location: &Path) -> Result<()> {
        Self::verify_location(location)?;
        let project = self.find_project(name)?;
        let premade_dir = self.options.get_premade_directory_path();
        let template_dir = self.options.get_template_directory_path();

        for step in project.steps {
            step.run(&premade_dir, &template_dir, location)?;
        }

        println!("Finished scaffolding your new {} project!", name.green());

        Ok(())
    }

    fn find_project(&self, name: &str) -> Result<ProjectScaffold> {
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

    fn verify_location(location: &Path) -> Result<()> {
        fs::create_dir_all(location)?;
        if let Some(_) = fs::read_dir(location)?.next() {
            println!("{}", "The specified directory is not empty".red());
            exit(1);
        }

        Ok(())
    }
}

impl Step {
    pub fn run(
        &self,
        premade_location: &Path,
        template_location: &Path,
        project_location: &Path,
    ) -> Result<()> {
        match self {
            Step::CopyFile { from, to } => {
                let src = premade_location.join(from);
                let dest = project_location.join(to);

                println!(
                    "Copying '{}' to '{}'.",
                    from.green(),
                    dest.to_string_lossy().green(),
                );

                fs::create_dir_all(dest.parent().ok_or("Couldn't find project dir")?)?;
                fs::copy(src, dest)?;

                Ok(())
            }
            Step::CreateFile { file, contents } => {
                let dest = project_location.join(file);

                println!("Creating file '{}'.", dest.to_string_lossy().green(),);

                fs::create_dir_all(dest.parent().ok_or("Couldn't find project dir")?)?;
                fs::write(dest, contents)?;

                Ok(())
            }
            Step::TemplateFile {
                template,
                file,
                replacements,
            } => {
                let src = template_location.join(template);
                let dest = project_location.join(file);

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
            Step::SingleCommand { command, args } => {
                println!(
                    "Running '{}{}{}'.",
                    command.green(),
                    if args.is_empty() { "" } else { " " },
                    args.join(" ").trim().green()
                );

                Command::new(command)
                    .args(args)
                    .current_dir(project_location.canonicalize()?)
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()?
                    .wait()?;

                Ok(())
            }
            Step::MultiCommand { command } => {
                println!("Running long command '{}'", command.green());
                let c_command = CString::new(command.as_bytes())?;

                let old_dir = std::env::current_dir()?;

                std::env::set_current_dir(project_location)?;
                unsafe {
                    system(c_command.as_ptr());
                }
                std::env::set_current_dir(old_dir)?;

                Ok(())
            }
            Step::AppendFile { file, contents } => {
                let dest = project_location.join(file);

                println!("Appending to file '{}'", dest.to_string_lossy().green());

                fs::File::options()
                    .append(true)
                    .open(dest)?
                    .write(contents.as_bytes())?;

                Ok(())
            }
            Step::RemoveFile { file } => {
                let file = project_location.join(file);

                println!("Deleting file '{}'", file.to_string_lossy().green());

                fs::remove_file(file)?;

                Ok(())
            }
        }
    }
}
