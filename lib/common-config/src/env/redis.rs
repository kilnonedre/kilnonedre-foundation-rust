use std::env;

use once_cell::sync::Lazy;

pub static REDIS_HOST: Lazy<String> =
    Lazy::new(|| env::var("REDIS_HOST").expect("❌ 必须设置环境变量 REDIS_HOST"));

pub static REDIS_PORT: Lazy<u64> = Lazy::new(|| {
    let val = env::var("REDIS_PORT").expect("❌ 必须设置环境变量 REDIS_PORT");
    val.parse::<u64>()
        .expect("❌ REDIS_PORT 必须是 0~65535 的数字")
});

pub static REDIS_PASSWORD: Lazy<String> =
    Lazy::new(|| env::var("REDIS_PASSWORD").expect("❌ 必须设置环境变量 REDIS_PASSWORD"));

pub static REDIS_DB: Lazy<u8> = Lazy::new(|| {
    let val = env::var("REDIS_DB").expect("❌ 必须设置环境变量 REDIS_DB");
    let db = val.parse::<u8>().expect("❌ REDIS_DB 必须是数字");
    if db > 15 {
        panic!("❌ REDIS_DB 必须是 0~15");
    }
    db
});
