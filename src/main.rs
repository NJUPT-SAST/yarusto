use anyhow::bail;
use yarusto::main_impl;

mod cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Err(e) = main_impl().await {
        bail!("{}", e);
    }
    Ok(())
}
