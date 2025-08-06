pub mod cli;
pub mod config;
pub mod data;
pub mod db;
pub mod server;

use rumqttc::QoS;

use crate::server::{receiver, sender};

pub const QOS: QoS = QoS::ExactlyOnce;

pub async fn async_main() -> anyhow::Result<()> {
    // 解析 命令行参数
    let run_mode = cli::parse_run_mode();
    match run_mode {
        RunMode::Sender => sender::main().await?,
        RunMode::Receiver => receiver::main().await?,
    }
    Ok(())
}

/// 运行模式是哪一方的
/// 就这玩意会有两种模式
/// 一种是压测机, 框框发请求
/// 一种是接收机, 接收 + insert 到 pg 里
pub enum RunMode {
    Sender,
    Receiver,
}

fn main() -> anyhow::Result<()> {
    config::init_config()?;

    println!("starting");

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(4) // 2c4g 总能跑得起来 4 线程吧，跑不起来那就是你的 CPU 不行（建议 fuck off cpu(
        .build()?;

    println!("building tokio runtime");

    rt.block_on(async_main())?;

    Ok(())
}
