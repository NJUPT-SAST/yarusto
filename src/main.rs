use anyhow::anyhow;
use yarusto::main_impl;

mod cli;

fn main() -> anyhow::Result<()> {
    if let Err(e) = main_impl() {
        return Err(anyhow!(e));
    }
    Ok(())
}
