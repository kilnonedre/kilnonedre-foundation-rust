use std::{collections::HashMap, sync::Arc};

use common_kafka::producer::KafkaProducer;
use common_ws::hub::WsHub;
// use redis::Client as RedisClient;

pub enum AppData {
    KafkaProducer(KafkaProducer),
    WsHub(Arc<WsHub>),
    // RedisClient(RedisClient),
}

pub type AppDataMap = HashMap<&'static str, AppData>;
