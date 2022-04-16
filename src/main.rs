#![warn(missing_docs)]

//! This is main entry point for this project

use cli::parse_args;
use anyhow::Result;
use app::start;

mod app;
mod cli;
mod config;
mod widget;
mod events;
mod draw;


fn main() -> Result<()> {
    let config = parse_args()?;

    start(&config)?;


    Ok(())
}
