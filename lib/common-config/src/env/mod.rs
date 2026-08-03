use std::env;

use once_cell::sync::Lazy;
use uuid::Uuid;

pub mod app_port;
pub mod grpc_host;
pub mod grpc_port;
pub mod kafka;
pub mod redis;
pub mod rust_fs;
pub mod we_chat;

pub static SVC_NAME: Lazy<String> =
    Lazy::new(|| env::var("SVC_NAME").expect("❌ 必须设置环境变量 SVC_NAME"));

pub static DB_SCHEMA: Lazy<String> =
    Lazy::new(|| env::var("DB_SCHEMA").expect("❌ 必须设置环境变量 DB_SCHEMA"));

pub static APP_PORT: Lazy<u16> = Lazy::new(|| {
    let val = env::var("APP_PORT").expect("❌ 必须设置环境变量 APP_PORT");
    val.parse::<u16>()
        .expect("❌ APP_PORT 必须是 0~65535 的数字")
});

pub static GRPC_PORT: Lazy<u16> = Lazy::new(|| {
    let val = env::var("GRPC_PORT").expect("❌ 必须设置环境变量 GRPC_PORT");
    val.parse::<u16>()
        .expect("❌ GRPC_PORT 必须是 0~65535 的数字")
});

pub static DB_HOST: Lazy<String> =
    Lazy::new(|| env::var("DB_HOST").expect("❌ 必须设置环境变量 DB_HOST"));

pub static DB_PORT: Lazy<String> =
    Lazy::new(|| env::var("DB_PORT").expect("❌ 必须设置环境变量 DB_PORT"));

pub static DB_USER: Lazy<String> =
    Lazy::new(|| env::var("DB_USER").expect("❌ 必须设置环境变量 DB_USER"));

pub static DB_PASSWORD: Lazy<String> =
    Lazy::new(|| env::var("DB_PASSWORD").expect("❌ 必须设置环境变量 DB_PASSWORD"));

pub static DB_NAME: Lazy<String> =
    Lazy::new(|| env::var("DB_NAME").expect("❌ 必须设置环境变量 DB_NAME"));

pub static SYS_ID: Lazy<Uuid> = Lazy::new(|| {
    let s = env::var("SYS_ID").expect("❌ 必须设置环境变量 SYS_ID");
    Uuid::parse_str(&s).expect("❌ SYS_ID 必须是合法的 UUID")
});

pub static LOG_LEVEL: Lazy<String> =
    Lazy::new(|| env::var("LOG_LEVEL").expect("❌ 必须设置环境变量 LOG_LEVEL"));

pub static ACCESS_EXPIRE_SECOND: Lazy<i64> = Lazy::new(|| {
    let val = env::var("ACCESS_EXPIRE_SECOND").expect("❌ 必须设置环境变量 ACCESS_EXPIRE_SECOND");
    val.parse::<i64>()
        .expect("❌ ACCESS_EXPIRE_SECOND 必须是 0~65535 的数字")
});

pub static ACCESS_SECRET: Lazy<String> =
    Lazy::new(|| env::var("ACCESS_SECRET").expect("❌ 必须设置环境变量 ACCESS_SECRET"));

pub static ACCESS_VERSION: Lazy<String> =
    Lazy::new(|| env::var("ACCESS_VERSION").expect("❌ 必须设置环境变量 ACCESS_VERSION"));
