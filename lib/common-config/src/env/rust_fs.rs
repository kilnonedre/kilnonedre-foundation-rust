use std::env;

use once_cell::sync::Lazy;

pub static RUSTFS_REGION: Lazy<String> =
    Lazy::new(|| env::var("RUSTFS_REGION").expect("❌ 必须设置环境变量 RUSTFS_REGION"));

pub static RUSTFS_ACCESS_KEY_ID: Lazy<String> = Lazy::new(|| {
    env::var("RUSTFS_ACCESS_KEY_ID").expect("❌ 必须设置环境变量 RUSTFS_ACCESS_KEY_ID")
});

pub static RUSTFS_SECRET_ACCESS_KEY: Lazy<String> = Lazy::new(|| {
    env::var("RUSTFS_SECRET_ACCESS_KEY").expect("❌ 必须设置环境变量 RUSTFS_SECRET_ACCESS_KEY")
});

pub static RUSTFS_ENDPOINT_URL: Lazy<String> =
    Lazy::new(|| env::var("RUSTFS_ENDPOINT_URL").expect("❌ 必须设置环境变量 RUSTFS_ENDPOINT_URL"));

pub static RUSTFS_BUCKET: Lazy<String> =
    Lazy::new(|| env::var("RUSTFS_BUCKET").expect("❌ 必须设置环境变量 RUSTFS_BUCKET"));
