use anyhow::Result;

use crate::{ConfigType, Vipera};

pub trait Configuration: serde::de::DeserializeOwned + serde::ser::Serialize {
    fn vipera() -> Vipera;

    fn read_in_config() -> Result<Self> {
        let vipera = Self::vipera();
        vipera.read_in_config()
    }

    fn write_config(&self) -> Result<()> {
        let vipera = Self::vipera();
        let paths = vipera
            .config_paths
            .iter()
            .map(|path| path.join(vipera.config_name.as_ref().unwrap()));
        let content = match vipera.config_type.as_ref().unwrap() {
            ConfigType::Toml => toml::to_string(&self)?,
            ConfigType::Yaml => serde_yaml::to_string(&self)?,
            ConfigType::Json => serde_json::to_string(&self)?,
        };
        for ref path in paths {
            let _ = std::fs::create_dir_all(path.parent().unwrap())
                .and_then(|_| std::fs::write(path, &content));
        }
        Ok(())
    }

    fn safe_write_config(&self) -> Result<()> {
        let vipera = Self::vipera();
        let paths = vipera
            .config_paths
            .iter()
            .map(|path| path.join(vipera.config_name.as_ref().unwrap()))
            .filter(|path| path.exists() && path.is_dir() || !path.exists());
        let content = match vipera.config_type.as_ref().unwrap() {
            ConfigType::Toml => toml::to_string(&self)?,
            ConfigType::Yaml => serde_yaml::to_string(&self)?,
            ConfigType::Json => serde_json::to_string(&self)?,
        };
        for ref path in paths {
            let _ = std::fs::create_dir_all(path.parent().unwrap())
                .and_then(|_| std::fs::write(path, &content));
        }
        Ok(())
    }
}
