use rand::Rng;
use rumqttc::{AsyncClient, MqttOptions};
/// 压测机
pub async fn main() -> anyhow::Result<()> { 
    let cfg = crate::config::get_cfg();

    let mut mqttoptions = MqttOptions::new(cfg.sender_id.clone(), cfg.broke_server_ip.clone(), cfg.broke_server_port);
    mqttoptions.set_keep_alive(std::time::Duration::from_secs(5));

    let (client, _) = AsyncClient::new(mqttoptions, cfg.client_capacity);
    client
        .subscribe(cfg.topic.clone(), crate::QOS)
        .await?;

    let mut rander = rand::rng();

    // tokio::task::spawn(async move {
    //     for i in 0..10 {
    //         client
    //             .publish("hello/rumqtt", crate::QOS, false, vec![i; i as usize])
    //             .await
    //             .unwrap();
    //         // time::sleep(Duration::from_millis(100)).await;
    //     }
    // });
    let mut counter = 0_u64;
    let start_tick = std::time::Instant::now();
    let mut count_tick = start_tick;

    loop {
        let rand_msg = crate::data::SendingData::new_rand(&mut rander);
        let byte_msg: Vec<u8> = (&rand_msg).into();
        client.publish(cfg.topic.clone(), crate::QOS, false, byte_msg).await?;
        counter += 1;
        if counter % 1000 == 0 {
            println!("已发送 {} 个数据", counter);
            let total_send_speed = counter as f64 / (start_tick.elapsed().as_nanos()) as f64 * 1e9;
            let count_time = count_tick.elapsed();
            let count_send_speed = counter as f64 / (count_time.as_nanos()) as f64 * 1e9;
            count_tick = std::time::Instant::now();
            // 1e9 是 1s 转换成 ns
            println!("总发送速度: {total_send_speed:.3} msg/s, 1k计时段发送速度: {count_send_speed:.3} 耗时: {count_time:?}");
        }
    }

    Ok(())
}
