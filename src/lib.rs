mod cli;
mod converter;
mod error;
mod model;

use clap::Parser;
use cli::Cli;
use converter::Converter;

pub async fn main_impl() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let input_path = cli.input_path;
    let output_path = cli.output_path;

    let converter = Converter::build(&input_path).await?;

    converter
        .rename()
        .await?
        .convert()
        .await?
        .tar(&output_path)
        .await?;

    Ok(())
}
