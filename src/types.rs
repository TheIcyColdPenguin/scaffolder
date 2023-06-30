use std::{collections::HashMap, path::PathBuf};

use clap::{Parser, Subcommand};
use serde::Deserialize;

pub type Result<T, E = dyn std::error::Error> = std::result::Result<T, Box<E>>;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct ScaffoldCliArgs {
    /// Choose a different config directory
    #[clap(short, long, value_name = "directory")]
    pub config: Option<PathBuf>,

    #[clap(subcommand)]
    pub command: CliCommands,
}

#[derive(Debug, Subcommand)]
pub enum CliCommands {
    /// Create a new project
    Create {
        /// Choose the kind of project - to see all projects, run `scaffolder list`
        #[clap(value_parser, value_name = "project kind")]
        name: String,
        /// Where to create the project
        #[clap(value_parser, value_name = "location")]
        location: PathBuf,
    },
    /// List available projects
    List,
}

#[derive(Debug)]
pub struct ScaffoldOptions {
    pub config: PathBuf,
    pub command: CliCommands,
}

#[derive(Debug, Deserialize)]
pub struct Projects {
    pub projects: Option<Vec<ProjectScaffold>>,
}

#[derive(Debug, Deserialize)]
pub struct ProjectScaffold {
    pub names: Vec<String>,
    pub description: String,
    pub steps: Vec<Step>,
}

#[derive(Debug, Deserialize)]
pub enum Step {
    #[serde(rename = "command")]
    SingleCommand { command: String, args: Vec<String> },

    #[serde(rename = "multicommand")]
    MultiCommand { command: String },

    #[serde(rename = "copy")]
    CopyFile { from: String, to: String },

    #[serde(rename = "template")]
    TemplateFile {
        template: String,
        file: String,
        replacements: HashMap<String, String>,
    },

    #[serde(rename = "append")]
    AppendFile { file: String, contents: String },

    #[serde(rename = "create")]
    CreateFile { file: String, contents: String },

    #[serde(rename = "remove")]
    RemoveFile { file: String },
}
