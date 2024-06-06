use std::path::{Path, PathBuf};

use config::ConfigError;
use rqs_lib::Visibility;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub visibility: Visibility,
    pub port_number: Option<u32>,
    pub download_path: Option<PathBuf>,
}

impl Config {
    pub fn new(config_dir: &Path) -> Result<Self, ConfigError> {
        if !config_dir.exists() {
            std::fs::create_dir_all(config_dir).expect("Failed to create config dir");
        }

        let file_path = config_dir.to_path_buf().join("config.toml");
        if !file_path.exists() {
            let default_config = Config::default();
            let contents = toml::to_string(&default_config)
                .expect("Failed to serialize default config to TOML");
            std::fs::write(&file_path, contents).expect("Failed to write default config file");
            info!("Default configuration written at {:?}", &file_path.to_str());
        }

        let config_builder = config::Config::builder().add_source(config::File::new(
            file_path.to_str().unwrap(),
            config::FileFormat::Toml,
        ));

        let config: Self = config_builder.build()?.try_deserialize()?;

        Ok(config)
    }
}
