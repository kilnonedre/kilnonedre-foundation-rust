use std::{collections::HashMap, sync::Arc};

use kilnonedre_common_kafka::producer::KafkaProducer;
use kilnonedre_common_ws::hub::WsHub;
// use redis::Client as RedisClient;

pub enum AppData {
    KafkaProducer(KafkaProducer),
    WsHub(Arc<WsHub>),
    // RedisClient(RedisClient),
}

pub type AppDataMap = HashMap<&'static str, AppData>;
