use once_cell::sync::Lazy;

use crate::util::load_env_str;

pub static KAFKA_BOOTSTRAP_SERVERS: Lazy<String> =
    Lazy::new(|| load_env_str("KAFKA_BOOTSTRAP_SERVERS"));

pub static KAFKA_GROUP_ID: Lazy<String> = Lazy::new(|| load_env_str("KAFKA_GROUP_ID"));
