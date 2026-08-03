use std::env;

use once_cell::sync::Lazy;

pub static KAFKA_BOOTSTRAP_SERVERS: Lazy<String> = Lazy::new(|| {
    env::var("KAFKA_BOOTSTRAP_SERVERS").expect("❌ 必须设置环境变量 KAFKA_BOOTSTRAP_SERVERS")
});

pub static KAFKA_GROUP_ID: Lazy<String> =
    Lazy::new(|| env::var("KAFKA_GROUP_ID").expect("❌ 必须设置环境变量 KAFKA_GROUP_ID"));
