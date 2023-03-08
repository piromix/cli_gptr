use anyhow::Result;
use tokio;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args = gptr::check_args();

    if args.is_err() {
        eprintln!("Error: {}", args.err().unwrap());
        std::process::exit(1);
    }

    if let Err(e) = gptr::run(args.unwrap()).await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}
