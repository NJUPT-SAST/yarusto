use anyhow::bail;
use yarusto::main_impl;

mod cli;

fn main() -> anyhow::Result<()> {
    if let Err(e) = main_impl() {
        bail!("Error: {}", e);
    }
    Ok(())
}
