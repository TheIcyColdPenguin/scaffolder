use app::App;
use clap::Parser;
use types::{Result, ScaffoldCliArgs};

mod app;
mod commands;
mod options;
mod types;
mod verify_config;

fn main() -> Result<()> {
    let args = ScaffoldCliArgs::parse();
    let app = App::new(args)?;
    app.process_command()?;

    Ok(())
}
