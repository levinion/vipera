mod config;

use std::{fs::read_to_string, path::PathBuf};

use anyhow::{bail, Context, Result};

pub use config::Configuration;

#[derive(Default)]
pub struct Vipera {
    config_name: Option<String>,
    config_paths: Vec<PathBuf>,
    config_type: Option<ConfigType>,
}

enum ConfigType {
    Toml,
    Yaml,
    Json,
}

impl Vipera {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_config_name(mut self, name: impl Into<String>) -> Result<Self> {
        let name: String = name.into();
        let path = PathBuf::from(&name);
        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_lowercase())
            .context(format!(
                "Could not determine a valid file extension for path: {:?}",
                path
            ))?;
        let config_type = match ext.as_str() {
            "toml" => ConfigType::Toml,
            "yaml" | "yml" => ConfigType::Yaml,
            "json" => ConfigType::Json,
            _ => bail!("Unsupported configuration format: .{}", ext),
        };
        self.config_name = Some(name);
        self.config_type = Some(config_type);
        Ok(self)
    }

    pub fn add_config_path(mut self, path: impl Into<String>) -> Result<Self> {
        let path_str = path.into();
        let path = PathBuf::from(shellexpand::full(&path_str)?.to_string());
        self.config_paths.push(path);
        Ok(self)
    }

    pub(crate) fn read_in_config<T: serde::de::DeserializeOwned>(&self) -> Result<T> {
        let config_path = self.get_config_file()?;
        let content = read_to_string(&config_path)?;
        let this = match self.config_type.as_ref().unwrap() {
            #[cfg(feature = "toml")]
            ConfigType::Toml => toml::from_str::<T>(&content)?,
            #[cfg(feature = "yaml")]
            ConfigType::Yaml => serde_yml::from_str::<T>(&content)?,
            #[cfg(feature = "json")]
            ConfigType::Json => serde_json::from_str::<T>(&content)?,
            #[allow(unreachable_patterns)]
            _ => bail!("The format is not supported by vipera yet, or not enabled in features"),
        };
        Ok(this)
    }

    pub(crate) fn get_config_file(&self) -> Result<PathBuf> {
        for path in &self.config_paths {
            let config_path = path.join(self.config_name.as_ref().unwrap());
            if config_path.is_file() {
                return Ok(config_path);
            }
        }
        bail!("The config file is not found");
    }
}
