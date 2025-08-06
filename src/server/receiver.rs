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

    const INSERT_BATCH: usize = 100;
    const COUNT_BATCH: usize = 10000;

    let mut buffer = Vec::with_capacity(INSERT_BATCH);
    while let Ok(notification) = eventloop.poll().await {
        if let Event::Incoming(rumqttc::Packet::Publish(p)) = notification {
            let data = p.payload.iter().as_slice();
            let data = SendingData::from(data);
            
            counter += 1;
            buffer.push(data);

            if counter.is_multiple_of(COUNT_BATCH as u64) {
                let total_secs = start_tick.elapsed().as_secs_f64();
                let total_tps = counter as f64 / total_secs;
                let interval_secs = count_tick.elapsed().as_secs_f64();
                let interval_tps = COUNT_BATCH as f64 / interval_secs;
                count_tick = Instant::now();
                println!(
                    "已接收 {} 条 | 总 TPS: {total_tps:.3} | 近 {COUNT_BATCH} TPS: {interval_tps:.3} | 耗时: {interval_secs:.3}s",
                    counter
                );
            }

            if counter.is_multiple_of(INSERT_BATCH as u64) {
                let conn = db_connection.clone();
                let buf = buffer.clone();
                tokio::spawn(async move { crate::db::insert_batch(&conn, &buf).await });
                buffer.clear();
            }
        }
    }

    Ok(())
}
