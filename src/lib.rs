mod config;

use std::{fs::read_to_string, path::PathBuf};

use anyhow::{anyhow, Context, Result};

pub use config::Configuration;

#[derive(Default)]
pub struct Vipera {
    config_name: Option<String>,
    config_paths: Vec<PathBuf>,
    config_type: Option<ConfigType>,
}

pub enum ConfigType {
    Toml,
    Yaml,
    Json,
}

impl Vipera {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_config_name(mut self, name: impl Into<String>) -> Self {
        let name: String = name.into();
        let path = PathBuf::from(&name);
        let config_type = match &path.extension().unwrap().to_str().unwrap().to_lowercase() as &str
        {
            "toml" => ConfigType::Toml,
            "yaml" => ConfigType::Yaml,
            "yml" => ConfigType::Yaml,
            "json" => ConfigType::Json,
            _ => unreachable!(),
        };
        self.config_name = Some(name);
        self.config_type = Some(config_type);
        self
    }

    pub fn add_config_path(mut self, path: impl Into<PathBuf>) -> Result<Self> {
        let path =
            shellexpand::full(path.into().to_str().context("invalid path str")?)?.to_string();
        let path: PathBuf = PathBuf::from(path);
        self.config_paths.push(path);
        Ok(self)
    }

    pub fn read_in_config<T: serde::de::DeserializeOwned>(&self) -> Result<T> {
        for path in &self.config_paths {
            let config_path = path.join(self.config_name.as_ref().unwrap());
            if config_path.is_file() {
                let content = read_to_string(&config_path)?;
                let this = match self.config_type.as_ref().unwrap() {
                    ConfigType::Toml => toml::from_str::<T>(&content)?,
                    ConfigType::Yaml => serde_yaml::from_str::<T>(&content)?,
                    ConfigType::Json => serde_json::from_str::<T>(&content)?,
                };
                return Ok(this);
            }
        }
        Err(anyhow!("CONFIG FILES NOT EXIST"))
    }
}
