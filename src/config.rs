use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

/// 全局配置
pub static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn get_cfg() -> &'static Config {
    CONFIG.get().unwrap()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DbCfg {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
    pub max_connections: u32,
}

impl Default for DbCfg {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 5432,
            user: "postgres".to_string(),
            password: "postgres".to_string(),
            database: "tps".to_string(),
            max_connections: 10,
        }
    }
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
    /// db stuf
    pub database: DbCfg,
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
            database: DbCfg::default(),
        }
    }
}

pub fn init_config() -> anyhow::Result<()> {
    let path = crate::cli::parse_config_path();

    let cfg = if let Some(path) = path {
        let cfg_str = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Failed to read your config file {path}"));
        toml::from_str(&cfg_str)?
    } else {
        match std::fs::read_to_string("config.toml") {
            Ok(cfg_str) => match toml::from_str(&cfg_str) {
                Ok(cfg) => cfg,
                Err(e) => {
                    panic!(
                        "Failed to read your config file config.toml, please check your config file. Error: {e}"
                    )
                }
            },
            Err(_) => {
                let cfg_str = toml::to_string(&Config::default())?;
                std::fs::write("config.toml", cfg_str).expect("Failed to write config.toml"); // 如果你看见这个消息，那就是你的操作系统干傻逼了*
                Config::default()
            }
        }
    };
    CONFIG.set(cfg).unwrap();

    Ok(())
}
