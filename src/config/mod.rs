use crate::cmus_status::output::Format;
use crate::error::prelude::*;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub const DEFAULT_CONFIG: &str = include_str!("../../config.toml");
const KEYWORD_CONFIG_DIR: &str = "<CONFIG_DIR>";
const CONFIG_FILES: [&str; 3] = [
    "./config.toml",
    "<CONFIG_DIR>/format.ron",
    "<CONFIG_DIR>/config.toml",
];

#[derive(Deserialize)]
pub struct Config {
    pub format: Format,
}

impl Config {
    /// RON or TOML string
    pub fn from_str(s: &str) -> MyResult<Self> {
        Self::from_ron(s).or_else(|_| Self::from_toml(s))
    }

    fn from_ron(ron: &str) -> MyResult<Self> {
        ron::de::from_str(ron)
            .map_err(|e| Error::FailedParsingConfig(None, e.to_string()))
    }

    fn from_toml(toml: &str) -> MyResult<Self> {
        toml::de::from_str(toml)
            .map_err(|e| Error::FailedParsingConfig(None, e.to_string()))
    }
}

pub fn get_config() -> MyResult<Config> {
    if let Some(conf_path) = get_config_file() {
        if let Ok(mut file) = File::open(&conf_path) {
            let mut file_content = String::new();
            file.read_to_string(&mut file_content).unwrap();
            Config::from_str(file_content.as_str()).map_err(|e| {
                if let Error::FailedParsingConfig(None, msg) = e {
                    Error::FailedParsingConfig(Some(conf_path), msg)
                } else {
                    e
                }
            })
        } else {
            default_config()
        }
    } else {
        default_config()
    }
}

fn default_config() -> MyResult<Config> {
    Config::from_toml(DEFAULT_CONFIG)
}

fn get_config_file() -> Option<PathBuf> {
    CONFIG_FILES.iter().find_map(|filepath| {
        let path = if filepath.starts_with(KEYWORD_CONFIG_DIR) {
            let filepath_without_keyword = filepath
                .replace(&format!("{}/", KEYWORD_CONFIG_DIR), "")
                .replace(KEYWORD_CONFIG_DIR, "");
            if let Some(mut path) = get_config_dir() {
                path.push(filepath_without_keyword);
                path
            } else {
                PathBuf::from(filepath_without_keyword)
            }
        } else {
            PathBuf::from(filepath)
        };
        if path.is_file() {
            Some(path)
        } else {
            None
        }
    })
}

fn get_config_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|mut d| {
        d.push(crate::meta::NAME);
        d
    })
}
