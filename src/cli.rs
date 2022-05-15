use clap::Parser;

use crate::{config::Config, theme::Theme};
use anyhow::Result;

#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about,
    long_about = "A TUI toolkit to view HelloGitHub"
)]
pub struct Args {
    #[clap(short, long, help = "配置文件路径")]
    pub path: Option<String>,

    #[clap(short, long, help = "是否显示帮助")]
    pub show_help: bool,

    #[clap(
        short,
        long,
        help = "终端样式，使用 --show-themes 查看内置样式列表",
        default_value = "darkcolorful"
    )]
    pub color_theme: Theme,

    #[clap(long, help = "显示内置样式列表")]
    pub show_themes: bool,
}

pub fn parse_args() -> Result<Config> {
    let args = Args::parse();
    Ok(Config::from(args))
}
