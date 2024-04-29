use anyhow::anyhow;
use yarusto::main_impl;

mod cli;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    if let Err(e) = main_impl().await {
        return Err(anyhow!(e));
    }
    Ok(())
}
