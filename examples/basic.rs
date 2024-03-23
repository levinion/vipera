#![allow(unused)]

#[derive(serde::Deserialize, Default)]
struct Config {
    pub scale: f64,
    pub cursor_size: Option<u32>,
    pub refresh: Option<i32>,
}

impl Config {
    pub fn new() -> Self {
        vipera::Vipera::new()
            .set_config_name("config.toml")
            .add_config_path("/etc/ura")
            .add_config_path("$HOME/.config/ura")
            .extract()
            .unwrap_or_default()
    }
}

fn main() {}
