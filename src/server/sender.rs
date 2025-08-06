use std::hint::black_box;

use rumqttc::{AsyncClient, MqttOptions};
/// 压测机
pub async fn main() -> anyhow::Result<()> { 
    let cfg = crate::config::get_cfg();

    println!("got cfg: {cfg:?}");

    let mut mqttoptions = MqttOptions::new(cfg.sender_id.clone(), cfg.broke_server_ip.clone(), cfg.broke_server_port);
    mqttoptions.set_keep_alive(std::time::Duration::from_secs(5));

    println!("connecting to {:?}", mqttoptions.broker_address());

    let (client, eventloop) = AsyncClient::new(mqttoptions, cfg.client_capacity);
    client
        .subscribe(cfg.topic.clone(), crate::QOS)
        .await.expect("faild to subscribe");

    println!("connected");

    let mut rander = rand::rng();

    let mut counter = 0_u64;
    let start_tick = std::time::Instant::now();
    let mut count_tick = start_tick;

    loop {
        let rand_msg = crate::data::SendingData::new_rand(&mut rander);
        let byte_msg: Vec<u8> = (&rand_msg).into();
        client.publish(cfg.topic.clone(), crate::QOS, true, byte_msg).await?;
        counter += 1;
        if counter % 10000 == 0 {
            println!("已发送 {} 个数据", counter);

            // 总体吞吐（从程序起点到现在）
            let total_secs = start_tick.elapsed().as_secs_f64();
            let total_tps = counter as f64 / total_secs;

            // 最近 10k 条的吞吐
            let interval_secs = count_tick.elapsed().as_secs_f64();
            let interval_tps = 10_000_f64 / interval_secs;

            // 更新时间戳
            count_tick = std::time::Instant::now();

            println!(
                "总发送速度: {total_tps:.3} msg/s, 10k计时段发送速度: {interval_tps:.3} msg/s, 耗时: {interval_secs:.3}s"
            );
        }
    }
    black_box(&eventloop);

    Ok(())
}
