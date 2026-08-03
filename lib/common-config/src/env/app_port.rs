use std::env;

use once_cell::sync::Lazy;

pub static APP_AUTH_PORT: Lazy<u16> = Lazy::new(|| {
    let val = env::var("APP_AUTH_PORT").expect("❌ 必须设置环境变量 APP_AUTH_PORT");
    val.parse::<u16>()
        .expect("❌ APP_AUTH_PORT 必须是 0~65535 的数字")
});

pub static APP_CRM_PORT: Lazy<u16> = Lazy::new(|| {
    let val = env::var("APP_CRM_PORT").expect("❌ 必须设置环境变量 APP_CRM_PORT");
    val.parse::<u16>()
        .expect("❌ APP_CRM_PORT 必须是 0~65535 的数字")
});

pub static APP_CATALOG_PORT: Lazy<u16> = Lazy::new(|| {
    let val = env::var("APP_CATALOG_PORT").expect("❌ 必须设置环境变量 APP_CATALOG_PORT");
    val.parse::<u16>()
        .expect("❌ APP_CATALOG_PORT 必须是 0~65535 的数字")
});

pub static APP_STORAGE_PORT: Lazy<u16> = Lazy::new(|| {
    let val = env::var("APP_STORAGE_PORT").expect("❌ 必须设置环境变量 APP_STORAGE_PORT");
    val.parse::<u16>()
        .expect("❌ APP_STORAGE_PORT 必须是 0~65535 的数字")
});

pub static APP_WORKFLOW_PORT: Lazy<u16> = Lazy::new(|| {
    let val = env::var("APP_WORKFLOW_PORT").expect("❌ 必须设置环境变量 APP_WORKFLOW_PORT");
    val.parse::<u16>()
        .expect("❌ APP_WORKFLOW_PORT 必须是 0~65535 的数字")
});

pub static APP_TRADE_PORT: Lazy<u16> = Lazy::new(|| {
    let val = env::var("APP_TRADE_PORT").expect("❌ 必须设置环境变量 APP_TRADE_PORT");
    val.parse::<u16>()
        .expect("❌ APP_TRADE_PORT 必须是 0~65535 的数字")
});

pub static APP_PAYMENT_PORT: Lazy<u16> = Lazy::new(|| {
    let val = env::var("APP_PAYMENT_PORT").expect("❌ 必须设置环境变量 APP_PAYMENT_PORT");
    val.parse::<u16>()
        .expect("❌ APP_PAYMENT_PORT 必须是 0~65535 的数字")
});

pub static APP_NOTIFICATION_PORT: Lazy<u16> = Lazy::new(|| {
    let val = env::var("APP_NOTIFICATION_PORT").expect("❌ 必须设置环境变量 APP_NOTIFICATION_PORT");
    val.parse::<u16>()
        .expect("❌ APP_NOTIFICATION_PORT 必须是 0~65535 的数字")
});

pub static APP_GEO_PORT: Lazy<u16> = Lazy::new(|| {
    let val = env::var("APP_GEO_PORT").expect("❌ 必须设置环境变量 APP_GEO_PORT");
    val.parse::<u16>()
        .expect("❌ APP_GEO_PORT 必须是 0~65535 的数字")
});

pub static APP_WMS_PORT: Lazy<u16> = Lazy::new(|| {
    let val = env::var("APP_WMS_PORT").expect("❌ 必须设置环境变量 APP_WMS_PORT");
    val.parse::<u16>()
        .expect("❌ APP_WMS_PORT 必须是 0~65535 的数字")
});

pub static APP_PROCUREMENT_PORT: Lazy<u16> = Lazy::new(|| {
    let val = env::var("APP_PROCUREMENT_PORT").expect("❌ 必须设置环境变量 APP_PROCUREMENT_PORT");
    val.parse::<u16>()
        .expect("❌ APP_PROCUREMENT_PORT 必须是 0~65535 的数字")
});
