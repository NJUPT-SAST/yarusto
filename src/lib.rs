mod cli;
mod converter;
mod model;

use clap::Parser;
use cli::Cli;
use converter::Converter;

pub async fn main_impl() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let input_path = cli.input_path;
    let output_path = cli.output_path;

    let converter = Converter::build(&input_path).await?;

    let convert_task = converter.convert();
    let rename_task = converter.rename();

    tokio::try_join!(convert_task, rename_task)?;

    converter.tar(&output_path).await?;

    Ok(())
}
