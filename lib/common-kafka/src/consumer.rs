use futures::StreamExt;
use log::{error, info};
use rdkafka::{
    consumer::{CommitMode, Consumer, StreamConsumer},
    message::Message,
    ClientConfig,
};
use serde::de::DeserializeOwned;

/// Kafka 消费者
pub type KafkaConsumer = StreamConsumer;

/// 创建 Kafka 消费者
pub fn build_consumer(
    bootstrap_servers: &str,
    group_id: &str,
) -> Result<KafkaConsumer, rdkafka::error::KafkaError> {
    ClientConfig::new()
        .set("bootstrap.servers", bootstrap_servers)
        .set("group.id", group_id)
        .set("enable.auto.commit", "false")
        .set("auto.offset.reset", "earliest")
        .create()
}

/// 订阅 JSON 消息
pub async fn subscribe_json<T, F, Fut>(
    consumer: KafkaConsumer,
    topics: &[&str],
    handler: F,
) -> Result<(), String>
where
    T: DeserializeOwned + Send + 'static,
    F: Fn(T) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = Result<(), String>> + Send,
{
    consumer.subscribe(topics).map_err(|e| e.to_string())?;

    info!("Kafka 消费者订阅成功: topics={:?}", topics);

    let mut stream = consumer.stream();

    while let Some(message) = stream.next().await {
        let message = match message {
            Ok(message) => message,
            Err(err) => {
                error!("Kafka 消息读取失败: {}", err);
                continue;
            }
        };

        let payload = match message.payload() {
            Some(payload) => payload,
            None => {
                consumer
                    .commit_message(&message, CommitMode::Async)
                    .map_err(|e| e.to_string())?;
                continue;
            }
        };

        let data: T = match serde_json::from_slice(payload) {
            Ok(data) => data,
            Err(err) => {
                error!("Kafka 消息反序列化失败: {}", err);

                consumer
                    .commit_message(&message, CommitMode::Async)
                    .map_err(|e| e.to_string())?;

                continue;
            }
        };

        if let Err(err) = handler(data).await {
            error!("Kafka 消息处理失败: {}", err);
            continue;
        }

        consumer
            .commit_message(&message, CommitMode::Async)
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}
