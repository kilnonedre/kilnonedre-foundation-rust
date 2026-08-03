use once_cell::sync::Lazy;
use redis::{aio::MultiplexedConnection, Client};

use crate::env::redis::{REDIS_DB, REDIS_HOST, REDIS_PASSWORD, REDIS_PORT};

pub static REDIS_CLIENT: Lazy<Client> = Lazy::new(|| {
    let url = format!(
        "redis://:{}@{}:{}/{}",
        *REDIS_PASSWORD, *REDIS_HOST, *REDIS_PORT, *REDIS_DB,
    );

    Client::open(url).expect("❌ Redis 初始化失败")
});

pub async fn build_redis_conn() -> MultiplexedConnection {
    REDIS_CLIENT
        .get_multiplexed_async_connection()
        .await
        .expect("❌ Redis 连接失败")
}
