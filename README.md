# Vipera

## Intro

Vipera is a straightforward Rust configuration crate, drawing inspiration from the functionality of Viper. Designed with simplicity in mind, it offers an easy-to-use approach for handling configurations in Rust applications. Whether you're working on a small project or a larger application, Vipera provides the tools needed to manage configuration settings efficiently. With features reminiscent of Viper, Vipera simplifies the process of accessing and manipulating configuration data, empowering developers to focus on building robust software solutions without getting bogged down in the intricacies of configuration management.

## Usage

```rust
use vipera::Configuration;

#[derive(serde::Deserialize, serde::Serialize, Default)]
struct Config {
    pub scale: f64,
    pub cursor_size: Option<u32>,
    pub refresh: Option<i32>,
}

impl vipera::Configuration for Config {
    fn vipera() -> vipera::Vipera {
        vipera::Vipera::new()
            .set_config_name("config.toml")
            .add_config_path("/etc/vipera")
            .add_config_path("$HOME/.config/vipera")
    }
}

fn main() {
    let config = Config::read_in_config().unwrap_or_default();
    config.write_config().unwrap();
}
```
