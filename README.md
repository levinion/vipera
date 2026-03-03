# Vipera

## Intro

Vipera is a straightforward Rust configuration crate, drawing inspiration from the functionality of Viper.

## Usage

```rust
use vipera::Configuration;

#[derive(serde::Deserialize, Default, Debug)]
struct Config {
    pub scale: f64,
    pub cursor_size: Option<u32>,
    pub refresh: Option<i32>,
}

impl vipera::Configuration for Config {
    fn vipera() -> vipera::Vipera {
        vipera::Vipera::new()
            .set_config_name("config.toml")
            .add_config_path("$HOME/.config/vipera")
            .add_config_path("/etc/vipera")
    }
}

fn main() {
    let config = Config::read_in_config().unwrap_or_default();
    dbg!(config);
}
```
