use tokio;
use anyhow::Result;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args = gptr::check_args()?;
    gptr::run(args).await?;
    Ok(())
}
