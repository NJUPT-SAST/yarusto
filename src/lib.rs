mod cli;
mod converter;

use clap::Parser;
use cli::Cli;
use converter::Converter;

pub async fn main_impl() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    let input_path = cli.input_path;
    let output_path = cli.output_path;
    Converter::build(&input_path)
        .await?
        .process()
        .await?
        .save(&output_path)
        .await?;
    Ok(())
}
