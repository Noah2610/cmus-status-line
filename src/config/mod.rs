mod output_config;

pub use output_config::OutputConfig;

use crate::cmus_status::status::Format;

pub fn get_config() -> Config {
    Config::default()
}

#[derive(Default)]
pub struct Config {
    pub format: Format,
    pub output: OutputConfig,
}
