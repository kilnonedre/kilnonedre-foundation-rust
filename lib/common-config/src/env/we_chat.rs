use once_cell::sync::Lazy;

use crate::util::load_env_str;

pub static WE_CHAT_APP_ID: Lazy<String> = Lazy::new(|| load_env_str("WE_CHAT_APP_ID"));

pub static WE_CHAT_APP_SECRET: Lazy<String> = Lazy::new(|| load_env_str("WE_CHAT_APP_SECRET"));

pub static WE_CHAT_APP_SESSION_URL: Lazy<String> =
    Lazy::new(|| load_env_str("WE_CHAT_APP_SESSION_URL"));

pub static WE_CHAT_APP_ACCESS_TOKEN_URL: Lazy<String> =
    Lazy::new(|| load_env_str("WE_CHAT_APP_ACCESS_TOKEN_URL"));

pub static WE_CHAT_APP_PHONE_NUMBER_URL: Lazy<String> =
    Lazy::new(|| load_env_str("WE_CHAT_APP_PHONE_NUMBER_URL"));

pub static WE_CHAT_PAY_MCH_ID: Lazy<String> = Lazy::new(|| load_env_str("WE_CHAT_PAY_MCH_ID"));

pub static WE_CHAT_PAY_SERIAL_NO: Lazy<String> =
    Lazy::new(|| load_env_str("WE_CHAT_PAY_SERIAL_NO"));

pub static WE_CHAT_PAY_PRIVATE_KEY_PATH: Lazy<String> =
    Lazy::new(|| load_env_str("WE_CHAT_PAY_PRIVATE_KEY_PATH"));

pub static WE_CHAT_PAY_NOTIFY_URL: Lazy<String> =
    Lazy::new(|| load_env_str("WE_CHAT_PAY_NOTIFY_URL"));

pub static WE_CHAT_REFUND_NOTIFY_URL: Lazy<String> =
    Lazy::new(|| load_env_str("WE_CHAT_REFUND_NOTIFY_URL"));

pub static WE_CHAT_PAY_API_V3_KEY: Lazy<String> =
    Lazy::new(|| load_env_str("WE_CHAT_PAY_API_V3_KEY"));

pub static WE_CHAT_PAY_PLATFORM_PUBLIC_KEY_PATH: Lazy<String> =
    Lazy::new(|| load_env_str("WE_CHAT_PAY_PLATFORM_PUBLIC_KEY_PATH"));
