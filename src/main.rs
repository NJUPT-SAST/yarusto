use yarusto::main_impl;

mod cli;

#[tokio::main]
async fn main() -> Result<(), tokio::io::Error> {
    main_impl().await?;
    Ok(())
}
