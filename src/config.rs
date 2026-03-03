use anyhow::Result;

use crate::Vipera;

pub trait Configuration: serde::de::DeserializeOwned {
    fn vipera() -> Vipera;

    fn read_in_config() -> Result<Self> {
        let vipera = Self::vipera();
        vipera.read_in_config()
    }
}
