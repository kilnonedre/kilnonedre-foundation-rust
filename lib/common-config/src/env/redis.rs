use once_cell::sync::Lazy;

use crate::util::{load_env_str, load_env_u64, load_env_u8};

pub static REDIS_HOST: Lazy<String> = Lazy::new(|| load_env_str("REDIS_HOST"));

pub static REDIS_PORT: Lazy<u64> = Lazy::new(|| load_env_u64("REDIS_PORT"));

pub static REDIS_PASSWORD: Lazy<String> = Lazy::new(|| load_env_str("REDIS_PASSWORD"));

pub static REDIS_DB: Lazy<u8> = Lazy::new(|| load_env_u8("REDIS_DB"));
