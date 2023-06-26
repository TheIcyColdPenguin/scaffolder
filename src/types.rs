use std::path::PathBuf;

use clap::{Parser, Subcommand};
use serde::Deserialize;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct ScaffoldCliArgs {
    #[clap(short, long, value_name = "directory")]
    pub config: Option<PathBuf>,

    #[clap(subcommand)]
    pub command: CliCommands,
}

#[derive(Debug, Subcommand)]
pub enum CliCommands {
    Create {
        #[clap(value_parser, value_name = "name")]
        name: String,
        #[clap(value_parser, value_name = "location")]
        location: PathBuf,
    },
    List,
}

#[derive(Debug)]
pub struct ScaffoldOptions {
    pub config: PathBuf,
    pub command: CliCommands,
}

#[derive(Debug, Clone)]
pub enum Commands {
    Create { name: String, location: PathBuf },
    List,
}

#[derive(Debug, Deserialize)]
pub struct ProjectScaffold {
    names: Vec<String>,
    description: String,
    commands: Vec<Command>,
}

#[derive(Debug, Deserialize)]
pub enum Command {
    SimpleCommand {
        command: String,
        args: Vec<String>,
    },
    CopyFile {
        old_file: String,
        new_file: String,
    },
    TextTemplate {
        file: String,
        replacements: Vec<String>,
    },
}

pub enum TextMod {
    Red,
    Green,
    Blue,
    Bold,
    Clear,
}
