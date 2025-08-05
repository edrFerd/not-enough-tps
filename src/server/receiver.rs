use rumqttc::{AsyncClient, MqttOptions, QoS};

pub async fn main() -> anyhow::Result<()> { 
    let cfg = crate::config::get_cfg();

    let mut mqttoptions = MqttOptions::new(cfg.sender_id.clone(), cfg.broke_server_ip.clone(), cfg.broke_server_port);
    mqttoptions.set_keep_alive(std::time::Duration::from_secs(5));

    let (_, mut eventloop) = AsyncClient::new(mqttoptions, cfg.client_capacity);
    client
        .subscribe(cfg.topic.clone(), crate::QOS)
        .await?;

    while let Ok(notification) = eventloop.poll().await {
        println!("Received = {notification:?}");
    }

    Ok(())
}
