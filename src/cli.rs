use clap::Parser;

use crate::config::Config;
use anyhow::Result;

#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about,
    long_about = "A TUI toolkit to view HelloGitHub"
)]
struct Args {
    #[clap(short, long, default_value = "I love HG", help = "Test name")]
    name: String,

    #[clap(short, long, default_value = ".", help = "Config's Path")]
    path: String,
}

pub fn parse_args() -> Result<Config> {
    let args = Args::parse();
    println!("{:?}", args);
    Ok(Config::default())
}
