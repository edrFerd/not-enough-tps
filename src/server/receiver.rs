use rumqttc::{AsyncClient, Event, MqttOptions};
use std::time::Instant;

use crate::data::SendingData;

pub async fn main() -> anyhow::Result<()> {
    let cfg = crate::config::get_cfg();

    let db_connection = crate::db::init_and_check_db().await?;

    let mut mqttoptions = MqttOptions::new(
        cfg.receiver_id.clone(),
        cfg.broke_server_ip.clone(),
        cfg.broke_server_port,
    );
    mqttoptions.set_keep_alive(std::time::Duration::from_secs(5));

    println!("connecting to {:?}", mqttoptions.broker_address());

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, cfg.client_capacity);
    client
        .subscribe(cfg.topic.clone(), crate::QOS)
        .await
        .expect("Failed to subscribe");

    let mut counter = 0_u64;
    let start_tick = Instant::now();
    let mut count_tick = start_tick;

    while let Ok(notification) = eventloop.poll().await {
        // println!("Received = {notification:?}");
        match notification {
            Event::Incoming(rumqttc::Packet::Publish(p)) => {
                let data = p.payload.iter().as_slice();
                let _ = SendingData::from(data);
                counter += 1;

                if counter % 10000 == 0 {
                    let total_secs = start_tick.elapsed().as_secs_f64();
                    let total_tps = counter as f64 / total_secs;
                    let interval_secs = count_tick.elapsed().as_secs_f64();
                    let interval_tps = 10_000_f64 / interval_secs;
                    count_tick = Instant::now();

                    println!(
                        "已接收 {} 条 | 总 TPS: {total_tps:.3} | 近 10k TPS: {interval_tps:.3} | 耗时: {interval_secs:.3}s",
                        counter
                    );
                }
            }
            _ => {}
        }
    }

    Ok(())
}
