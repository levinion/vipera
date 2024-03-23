use std::{fs::read_to_string, path::PathBuf};

use anyhow::{anyhow, Result};

#[derive(Default)]
pub struct Vipera {
    config_name: Option<String>,
    config_paths: Vec<PathBuf>,
}

impl Vipera {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_config_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.config_name = Some(name.into());
        self
    }

    pub fn add_config_path(&mut self, path: impl Into<PathBuf>) -> &mut Self {
        let path: PathBuf = path.into();
        let path = path.into_os_string().into_string().unwrap();
        let home_dir = std::env::var("HOME").unwrap();
        let path = path.replace("$HOME", &home_dir).replace('~', &home_dir);
        self.config_paths.push(path.into());
        self
    }

    pub fn extract<T: serde::de::DeserializeOwned>(&self) -> Result<T> {
        for path in &self.config_paths {
            let config_path = path.join(self.config_name.as_ref().unwrap());
            if config_path.is_file() {
                let content = read_to_string(&config_path)?;
                return Ok(toml::from_str::<T>(&content)?);
            }
        }
        Err(anyhow!("CONFIG FILES NOT EXIST"))
    }
}
