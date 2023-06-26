use app::App;
use clap::Parser;
use types::ScaffoldCliArgs;

mod app;
mod types;
mod verify_config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = ScaffoldCliArgs::parse();
    let app = App::new(args)?;
    println!("{:#?}", app);

    Ok(())
}
