use std::sync::OnceLock;

use serde::{Serialize, Deserialize};

/// 全局配置
pub static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn get_cfg() -> &'static Config {
    CONFIG.get().unwrap()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub server_ip: String,
    pub server_port: u16,
    pub topic: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server_ip: "0.0.0.0".to_string(),
            server_port: 1883,
            topic: "not/enough/tps".to_string(),
        }
    }
}


pub fn init_config() -> anyhow::Result<()> {
    // TODO: Load config from file
    let path = crate::cli::parse_config_path();

    let cfg = if let Some(path) = path {
        let cfg_str = std::fs::read_to_string(path)?;
        toml::from_str(&cfg_str)?
    } else {
        Config::default()
    };
    CONFIG.set(cfg).unwrap();

    Ok(())
}