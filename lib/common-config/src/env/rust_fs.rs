use once_cell::sync::Lazy;

use crate::util::load_env_str;

pub static RUSTFS_REGION: Lazy<String> = Lazy::new(|| load_env_str("RUSTFS_REGION"));

pub static RUSTFS_ACCESS_KEY_ID: Lazy<String> = Lazy::new(|| load_env_str("RUSTFS_ACCESS_KEY_ID"));

pub static RUSTFS_SECRET_ACCESS_KEY: Lazy<String> =
    Lazy::new(|| load_env_str("RUSTFS_SECRET_ACCESS_KEY"));

pub static RUSTFS_ENDPOINT_URL: Lazy<String> = Lazy::new(|| load_env_str("RUSTFS_ENDPOINT_URL"));

pub static RUSTFS_BUCKET: Lazy<String> = Lazy::new(|| load_env_str("RUSTFS_BUCKET"));
