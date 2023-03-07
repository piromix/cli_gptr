use clap::Parser;
use anyhow::Result;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The prompt to use for the GPT-3.5 query. If no provided, the prompt will be read from stdin.
    #[arg(default_value_t = String::from("-"))]
    pub prompt: String,

    /// translate prompt
    #[arg(short, long)]
    pub translate: bool,

    /// use user content before the prompt
    #[arg(short, long)]
    pub predefine: Option<String>,

}

impl Args {
    pub fn get_args() -> Result<Args> {
        return Ok(Args::parse());
    }
}