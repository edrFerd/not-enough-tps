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

    let rander = rand::rng();

    // tokio::task::spawn(async move {
    //     for i in 0..10 {
    //         client
    //             .publish("hello/rumqtt", crate::QOS, false, vec![i; i as usize])
    //             .await
    //             .unwrap();
    //         // time::sleep(Duration::from_millis(100)).await;
    //     }
    // });

    loop {
        let rand_msg = crate::data::SendingData::new_rand(&mut rander);
    }

    Ok(())
}
