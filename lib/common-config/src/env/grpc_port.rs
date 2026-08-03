use std::env;

use once_cell::sync::Lazy;

pub static GRPC_AUTH_PORT: Lazy<u16> = Lazy::new(|| {
    let val = env::var("GRPC_AUTH_PORT").expect("❌ 必须设置环境变量 GRPC_AUTH_PORT");
    val.parse::<u16>()
        .expect("❌ GRPC_AUTH_PORT 必须是 0~65535 的数字")
});

pub static GRPC_CRM_PORT: Lazy<u16> = Lazy::new(|| {
    let val = env::var("GRPC_CRM_PORT").expect("❌ 必须设置环境变量 GRPC_CRM_PORT");
    val.parse::<u16>()
        .expect("❌ GRPC_CRM_PORT 必须是 0~65535 的数字")
});

pub static GRPC_CATALOG_PORT: Lazy<u16> = Lazy::new(|| {
    let val = env::var("GRPC_CATALOG_PORT").expect("❌ 必须设置环境变量 GRPC_CATALOG_PORT");
    val.parse::<u16>()
        .expect("❌ GRPC_CATALOG_PORT 必须是 0~65535 的数字")
});

pub static GRPC_STORAGE_PORT: Lazy<u16> = Lazy::new(|| {
    let val = env::var("GRPC_STORAGE_PORT").expect("❌ 必须设置环境变量 GRPC_STORAGE_PORT");
    val.parse::<u16>()
        .expect("❌ GRPC_STORAGE_PORT 必须是 0~65535 的数字")
});

pub static GRPC_WORKFLOW_PORT: Lazy<u16> = Lazy::new(|| {
    let val = env::var("GRPC_WORKFLOW_PORT").expect("❌ 必须设置环境变量 GRPC_WORKFLOW_PORT");
    val.parse::<u16>()
        .expect("❌ GRPC_WORKFLOW_PORT 必须是 0~65535 的数字")
});

pub static GRPC_TRADE_PORT: Lazy<u16> = Lazy::new(|| {
    let val = env::var("GRPC_TRADE_PORT").expect("❌ 必须设置环境变量 GRPC_TRADE_PORT");
    val.parse::<u16>()
        .expect("❌ GRPC_TRADE_PORT 必须是 0~65535 的数字")
});

pub static GRPC_PAYMENT_PORT: Lazy<u16> = Lazy::new(|| {
    let val = env::var("GRPC_PAYMENT_PORT").expect("❌ 必须设置环境变量 GRPC_PAYMENT_PORT");
    val.parse::<u16>()
        .expect("❌ GRPC_PAYMENT_PORT 必须是 0~65535 的数字")
});

pub static GRPC_NOTIFICATION_PORT: Lazy<u16> = Lazy::new(|| {
    let val =
        env::var("GRPC_NOTIFICATION_PORT").expect("❌ 必须设置环境变量 GRPC_NOTIFICATION_PORT");
    val.parse::<u16>()
        .expect("❌ GRPC_NOTIFICATION_PORT 必须是 0~65535 的数字")
});

pub static GRPC_GEO_PORT: Lazy<u16> = Lazy::new(|| {
    let val = env::var("GRPC_GEO_PORT").expect("❌ 必须设置环境变量 GRPC_GEO_PORT");
    val.parse::<u16>()
        .expect("❌ GRPC_GEO_PORT 必须是 0~65535 的数字")
});

pub static GRPC_LOGISTIC_PORT: Lazy<u16> = Lazy::new(|| {
    let val = env::var("GRPC_LOGISTIC_PORT").expect("❌ 必须设置环境变量 GRPC_LOGISTIC_PORT");
    val.parse::<u16>()
        .expect("❌ GRPC_LOGISTIC_PORT 必须是 0~65535 的数字")
});

pub static GRPC_WMS_PORT: Lazy<u16> = Lazy::new(|| {
    let val = env::var("GRPC_WMS_PORT").expect("❌ 必须设置环境变量 GRPC_WMS_PORT");
    val.parse::<u16>()
        .expect("❌ GRPC_WMS_PORT 必须是 0~65535 的数字")
});

pub static GRPC_PROCUREMENT_PORT: Lazy<u16> = Lazy::new(|| {
    let val = env::var("GRPC_PROCUREMENT_PORT").expect("❌ 必须设置环境变量 GRPC_PROCUREMENT_PORT");
    val.parse::<u16>()
        .expect("❌ GRPC_PROCUREMENT_PORT 必须是 0~65535 的数字")
});
