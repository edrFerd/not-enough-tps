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
    /// MQTT 主题
    pub topic: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            broke_server_ip: "192.168.3.45".to_string(),
            broke_server_port: 1883,
            sender_id: "net-sender".to_string(),
            receiver_id: "net-receiver".to_string(),
            client_capacity: 100,
            topic: "not-enough/tps".to_string(),
        }
    }
}


pub fn init_config() -> anyhow::Result<()> {
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