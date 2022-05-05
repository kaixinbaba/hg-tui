#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct Config {
    pub moyu: bool,
    pub config_path: String,
    pub show_help: bool,
}

impl From<crate::cli::Args> for Config {
    fn from(args: crate::cli::Args) -> Self {
        let config_path = args.path.unwrap_or_else(|| env!("HOME").to_string());
        Config {
            moyu: args.moyu,
            config_path,
            show_help: args.show_help,
        }
    }
}
