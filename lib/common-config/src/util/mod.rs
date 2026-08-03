use std::env;

use uuid::Uuid;

pub fn load_env_str(key: &str) -> String {
    env::var(key).expect(&format!("❌ 必须设置环境变量 {}", key))
}

pub fn load_env_i64(key: &str) -> i64 {
    let val = env::var(key).expect(&format!("❌ 必须设置环境变量 {}", key));
    val.parse::<i64>()
        .expect(&format!("❌ {} 必须是 0~65535 的数字", key))
}

pub fn load_env_u8(key: &str) -> u8 {
    let val = env::var(key).expect(&format!("❌ 必须设置环境变量 {}", key));
    val.parse::<u8>()
        .expect(&format!("❌ {} 必须是 0~65535 的数字", key))
}

pub fn load_env_u16(key: &str) -> u16 {
    let val = env::var(key).expect(&format!("❌ 必须设置环境变量 {}", key));
    val.parse::<u16>()
        .expect(&format!("❌ {} 必须是 0~65535 的数字", key))
}

pub fn load_env_u64(key: &str) -> u64 {
    let val = env::var(key).expect(&format!("❌ 必须设置环境变量 {}", key));
    val.parse::<u64>()
        .expect(&format!("❌ {} 必须是 0~65535 的数字", key))
}

pub fn load_env_uuid(key: &str) -> Uuid {
    let val = env::var(key).expect(&format!("❌ 必须设置环境变量 {}", key));
    Uuid::parse_str(&val).expect(&format!("❌ {} 必须是合法的 UUID", key))
}
