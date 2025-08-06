use std::sync::OnceLock;

use serde::{Serialize, Deserialize};

/// 全局配置
pub static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn get_cfg() -> &'static Config {
    CONFIG.get().unwrap()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// MQTT 服务器地址
    pub broke_server_ip: String,
    /// MQTT 端口
    pub broke_server_port: u16,
    /// 压测机 id
    pub sender_id: String,
    /// 接收机 id
    pub receiver_id: String,
    /// 客户端容量
    pub client_capacity: usize,
    /// 是否 retain
    pub retain: bool,
    /// MQTT 主题
    pub topic: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            broke_server_ip: "127.0.0.1".to_string(),
            broke_server_port: 1884,
            sender_id: "net-sender".to_string(),
            receiver_id: "net-receiver".to_string(),
            client_capacity: 100,
            retain: false,
            topic: "notenough/tps".to_string(),
        }
    }
}


pub fn init_config() -> anyhow::Result<()> {
    let path = crate::cli::parse_config_path();

    let cfg = if let Some(path) = path {
        let cfg_str = std::fs::read_to_string(path)?;
        toml::from_str(&cfg_str)?
    } else {
        // let cfg_str = toml::to_string(&Config::default())?;
        // std::fs::write("config.toml", cfg_str)?;
        // Config::default()
        match std::fs::read_to_string("config.toml") {
            Ok(cfg_str) => toml::from_str(&cfg_str)?,
            Err(_) => {
                let cfg_str = toml::to_string(&Config::default())?;
                std::fs::write("config.toml", cfg_str)?;
                Config::default()
            }

        }
    };
    CONFIG.set(cfg).unwrap();

    Ok(())
}