use app::App;
use clap::Parser;
use types::ScaffoldArgs;

mod app;
mod types;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = ScaffoldArgs::parse();
    let app = App::new(args)?;
    println!("{:#?}", app);

    Ok(())
}
