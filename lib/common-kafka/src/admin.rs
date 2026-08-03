use std::time::Duration;

use rdkafka::{
    admin::{AdminClient, AdminOptions, NewTopic, TopicReplication},
    client::DefaultClientContext,
    error::{KafkaError, RDKafkaErrorCode},
    ClientConfig,
};

pub type KafkaAdminClient = AdminClient<DefaultClientContext>;

pub fn build_admin_client(bootstrap_servers: &str) -> Result<KafkaAdminClient, KafkaError> {
    ClientConfig::new()
        .set("bootstrap.servers", bootstrap_servers)
        .create()
}

pub async fn create_topics_if_not_exists(
    admin: &KafkaAdminClient,
    topics: &[&str],
) -> Result<(), KafkaError> {
    let new_topics = topics
        .iter()
        .map(|topic| NewTopic::new(topic, 1, TopicReplication::Fixed(1)))
        .collect::<Vec<_>>();

    let result = admin
        .create_topics(
            &new_topics,
            &AdminOptions::new().operation_timeout(Some(Duration::from_secs(10))),
        )
        .await?;

    for item in result {
        match item {
            Ok(topic) => {
                log::info!("Kafka Topic 创建成功: {}", topic);
            }

            Err((topic, RDKafkaErrorCode::TopicAlreadyExists)) => {
                log::info!("Kafka Topic 已存在: {}", topic);
            }

            Err((topic, err)) => {
                log::error!("Kafka Topic 创建失败: {}, 错误: {:?}", topic, err);
            }
        }
    }

    Ok(())
}

pub async fn init_kafka_topics(bootstrap_servers: &str, topics: &[&str]) -> std::io::Result<()> {
    let admin = build_admin_client(bootstrap_servers)
        .map_err(|e| std::io::Error::other(format!("Kafka 服务创建失败: {}", e)))?;

    create_topics_if_not_exists(&admin, topics)
        .await
        .map_err(|e| std::io::Error::other(format!("Kafka Topic 创建失败: {}", e)))?;

    Ok(())
}
