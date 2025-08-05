pub mod server;
pub mod data;
pub mod config;
pub mod cli;

use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::time::Duration;
use tokio::{task, time};

pub async fn async_main() -> anyhow::Result<()> {
    println!("Hello, world!");

    // let mut mqttoptions = MqttOptions::new("rumqtt-async", "test.mosquitto.org", 1883);
    let mut mqttoptions = MqttOptions::new("rumqtt-async", "192.168.3.45", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client
        .subscribe("hello/rumqtt", QoS::AtMostOnce)
        .await
        .unwrap();

    task::spawn(async move {
        for i in 0..10 {
            client
                .publish("hello/rumqtt", QoS::AtLeastOnce, false, vec![i; i as usize])
                .await
                .unwrap();
            // time::sleep(Duration::from_millis(100)).await;
        }
    });

    while let Ok(notification) = eventloop.poll().await {
        println!("Received = {notification:?}");
    }
    Ok(())
}

/// 运行模式是哪一方的
/// 就这玩意会有两种模式
/// 一种是压测机, 框框发请求
/// 一种是接收机, 接受+insert到pg里
pub enum RunMode {
    Sender,
    Receiver,
}

fn main() -> anyhow::Result<()> {
    // 解析 命令行参数
    let run_mode = cli::parse_run_mode();
    config::init_config()?;

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(4) // 2c4g 总能跑得起来 4 线程吧，跑不起来那就是你的 CPU 不行（建议 fuck off cpu(
        .build()?;

    rt.block_on(async_main())?;

    Ok(())
}
