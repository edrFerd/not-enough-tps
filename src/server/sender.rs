use std::time::Instant;

use rumqttc::{AsyncClient, MqttOptions};
use anyhow::Result;


pub async fn notification(
    mut eventloop: rumqttc::EventLoop
) {
    while let Ok(_) = eventloop.poll().await {
        
    }
}

pub async fn send_main(client: AsyncClient) -> Result<()> {
    let cfg = crate::config::get_cfg();
    
    let mut rander = rand::rng();
    let mut counter = 0_u64;
    let start_tick = Instant::now();
    let mut count_tick = start_tick;

    loop {
        // 生成并发布消息
        let rand_msg = crate::data::SendingData::new_rand(&mut rander);
        let byte_msg: Vec<u8> = (&rand_msg).into();
        client
            .publish(&cfg.topic, crate::QOS, cfg.retain, byte_msg)
            .await?;
        counter += 1;

        // 3. 高频处理事件循环（重点处理PubAck，释放发送窗口）
        // 非阻塞轮询，避免阻塞发布流程

        // 打印统计信息（每10k条）
        if counter % 10000 == 0 {
            let total_secs = start_tick.elapsed().as_secs_f64();
            let total_tps = counter as f64 / total_secs;
            let interval_secs = count_tick.elapsed().as_secs_f64();
            let interval_tps = 10_000_f64 / interval_secs;
            count_tick = Instant::now();

            println!(
                "已发送 {} 条 | 总TPS: {total_tps:.3} | 近10k TPS: {interval_tps:.3} | 耗时: {interval_secs:.3}s",
                counter
            );
        }
    }
}

/// 压测机（发送端）
pub async fn main() -> Result<()> {
    let cfg = crate::config::get_cfg();
    println!("got cfg: {cfg:?}");

    // 1. 配置MQTT客户端（发送端不需要订阅，无需多余操作）
    let mut mqttoptions = MqttOptions::new(
        cfg.sender_id.clone(),
        cfg.broke_server_ip.clone(),
        cfg.broke_server_port,
    );
    mqttoptions.set_keep_alive(std::time::Duration::from_secs(5));

    println!("connecting to {:?}", mqttoptions.broker_address());

    // 2. 创建客户端（建议增大client_capacity，如10万，避免缓冲区满）
    let (client, eventloop) = AsyncClient::new(mqttoptions, cfg.client_capacity);


    println!("connected, start publishing...");

    let sender = tokio::spawn(notification(eventloop));
    
    // let publish = tokio::spawn(send_main(client));
    
    send_main(client).await?;


    sender.await?;

    Ok(())
}
    