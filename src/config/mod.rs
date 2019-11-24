mod output_config;

pub use output_config::OutputConfig;

use crate::cmus_status::status::Format;
use crate::error::prelude::*;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

const CONFIG_FILE: &str = "cmus-status-line/config.toml";

pub fn get_config() -> MyResult<Config> {
    if let Some(mut conf_path) = dirs::config_dir() {
        conf_path.push(CONFIG_FILE);
        if let Ok(mut file) = File::open(&conf_path) {
            let mut file_content = String::new();
            file.read_to_string(&mut file_content).unwrap();
            toml::de::from_str(file_content.as_str()).or_else(|e| {
                Err(Error::FailedParsingConfig(conf_path, e.to_string()))
            })
        } else {
            Ok(Config::default())
        }
    } else {
        Ok(Config::default())
    }
}

#[derive(Default, Deserialize)]
pub struct Config {
    pub format: Format,
    pub output: OutputConfig,
}
