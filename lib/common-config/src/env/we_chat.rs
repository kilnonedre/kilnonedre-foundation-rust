use std::env;

use once_cell::sync::Lazy;

pub static WE_CHAT_APP_ID: Lazy<String> =
    Lazy::new(|| env::var("WE_CHAT_APP_ID").expect("❌ 必须设置环境变量 WE_CHAT_APP_ID"));

pub static WE_CHAT_APP_SECRET: Lazy<String> =
    Lazy::new(|| env::var("WE_CHAT_APP_SECRET").expect("❌ 必须设置环境变量 WE_CHAT_APP_SECRET"));

pub static WE_CHAT_APP_SESSION_URL: Lazy<String> = Lazy::new(|| {
    env::var("WE_CHAT_APP_SESSION_URL").expect("❌ 必须设置环境变量 WE_CHAT_APP_SESSION_URL")
});

pub static WE_CHAT_APP_ACCESS_TOKEN_URL: Lazy<String> = Lazy::new(|| {
    env::var("WE_CHAT_APP_ACCESS_TOKEN_URL")
        .expect("❌ 必须设置环境变量 WE_CHAT_APP_ACCESS_TOKEN_URL")
});

pub static WE_CHAT_APP_PHONE_NUMBER_URL: Lazy<String> = Lazy::new(|| {
    env::var("WE_CHAT_APP_PHONE_NUMBER_URL")
        .expect("❌ 必须设置环境变量 WE_CHAT_APP_PHONE_NUMBER_URL")
});

pub static WE_CHAT_PAY_MCH_ID: Lazy<String> =
    Lazy::new(|| env::var("WE_CHAT_PAY_MCH_ID").expect("❌ 必须设置环境变量 WE_CHAT_PAY_MCH_ID"));

pub static WE_CHAT_PAY_SERIAL_NO: Lazy<String> = Lazy::new(|| {
    env::var("WE_CHAT_PAY_SERIAL_NO").expect("❌ 必须设置环境变量 WE_CHAT_PAY_SERIAL_NO")
});

pub static WE_CHAT_PAY_PRIVATE_KEY_PATH: Lazy<String> = Lazy::new(|| {
    env::var("WE_CHAT_PAY_PRIVATE_KEY_PATH")
        .expect("❌ 必须设置环境变量 WE_CHAT_PAY_PRIVATE_KEY_PATH")
});

pub static WE_CHAT_PAY_NOTIFY_URL: Lazy<String> = Lazy::new(|| {
    env::var("WE_CHAT_PAY_NOTIFY_URL").expect("❌ 必须设置环境变量 WE_CHAT_PAY_NOTIFY_URL")
});

pub static WE_CHAT_REFUND_NOTIFY_URL: Lazy<String> = Lazy::new(|| {
    env::var("WE_CHAT_REFUND_NOTIFY_URL").expect("❌ 必须设置环境变量 WE_CHAT_REFUND_NOTIFY_URL")
});

pub static WE_CHAT_PAY_API_V3_KEY: Lazy<String> = Lazy::new(|| {
    env::var("WE_CHAT_PAY_API_V3_KEY").expect("❌ 必须设置环境变量 WE_CHAT_PAY_API_V3_KEY")
});

pub static WE_CHAT_PAY_PLATFORM_PUBLIC_KEY_PATH: Lazy<String> = Lazy::new(|| {
    env::var("WE_CHAT_PAY_PLATFORM_PUBLIC_KEY_PATH")
        .expect("❌ 必须设置环境变量 WE_CHAT_PAY_PLATFORM_PUBLIC_KEY_PATH")
});
