use std::time::Duration;

use kilnonedre_common_web::{util::error::svc_err_internal, ApiError};
use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    ClientConfig,
};
use serde::Serialize;

pub type KafkaProducer = FutureProducer;

pub fn build_producer(bootstrap_servers: &str) -> Result<KafkaProducer, std::io::Error> {
    ClientConfig::new()
        .set("bootstrap.servers", bootstrap_servers)
        .set("message.timeout.ms", "5000")
        .create()
        .map_err(|e| std::io::Error::other(format!("Kafka Producer 创建失败: {}", e)))
}

pub async fn send_json<T>(
    producer: &KafkaProducer,
    topic: &str,
    key: &str,
    payload: &T,
) -> Result<(), ApiError>
where
    T: Serialize,
{
    let body =
        serde_json::to_string(payload).map_err(|e| svc_err_internal(e, "Json 序列化失败"))?;

    producer
        .send(
            FutureRecord::to(topic).key(key).payload(&body),
            Duration::from_secs(5),
        )
        .await
        .map_err(|(e, _)| svc_err_internal(e, "Kafka 消息发送失败"))?;

    Ok(())
}
