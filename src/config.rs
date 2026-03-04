use std::path::PathBuf;

use anyhow::Result;

use crate::Vipera;

pub trait Configuration: serde::de::DeserializeOwned {
    fn vipera() -> Result<Vipera>;

    fn read_in_config() -> Result<Self> {
        let vipera = Self::vipera()?;
        vipera.read_in_config()
    }

    fn get_config_file() -> Result<PathBuf> {
        let vipera = Self::vipera()?;
        vipera.get_config_file()
    }
}
