mod cli;

use clap::Parser;
use cli::Cli;

pub async fn main_impl() -> Result<(), tokio::io::Error> {
    let cli = Cli::parse();

    let input_path = cli.input_path;
    let output_path = cli.output_path;
    Ok(())
}
