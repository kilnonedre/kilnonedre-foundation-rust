use std::env;

use once_cell::sync::Lazy;

pub static GRPC_AUTH_HOST: Lazy<String> =
    Lazy::new(|| env::var("GRPC_AUTH_HOST").expect("❌ 必须设置环境变量 GRPC_AUTH_HOST"));

pub static GRPC_CRM_HOST: Lazy<String> =
    Lazy::new(|| env::var("GRPC_CRM_HOST").expect("❌ 必须设置环境变量 GRPC_CRM_HOST"));

pub static GRPC_CATALOG_HOST: Lazy<String> =
    Lazy::new(|| env::var("GRPC_CATALOG_HOST").expect("❌ 必须设置环境变量 GRPC_CATALOG_HOST"));

pub static GRPC_STORAGE_HOST: Lazy<String> =
    Lazy::new(|| env::var("GRPC_STORAGE_HOST").expect("❌ 必须设置环境变量 GRPC_STORAGE_HOST"));

pub static GRPC_WORKFLOW_HOST: Lazy<String> =
    Lazy::new(|| env::var("GRPC_WORKFLOW_HOST").expect("❌ 必须设置环境变量 GRPC_WORKFLOW_HOST"));

pub static GRPC_TRADE_HOST: Lazy<String> =
    Lazy::new(|| env::var("GRPC_TRADE_HOST").expect("❌ 必须设置环境变量 GRPC_TRADE_HOST"));

pub static GRPC_PAYMENT_HOST: Lazy<String> =
    Lazy::new(|| env::var("GRPC_PAYMENT_HOST").expect("❌ 必须设置环境变量 GRPC_PAYMENT_HOST"));

pub static GRPC_NOTIFICATION_HOST: Lazy<String> = Lazy::new(|| {
    env::var("GRPC_NOTIFICATION_HOST").expect("❌ 必须设置环境变量 GRPC_NOTIFICATION_HOST")
});

pub static GRPC_GEO_HOST: Lazy<String> =
    Lazy::new(|| env::var("GRPC_GEO_HOST").expect("❌ 必须设置环境变量 GRPC_GEO_HOST"));

pub static GRPC_LOGISTIC_HOST: Lazy<String> =
    Lazy::new(|| env::var("GRPC_LOGISTIC_HOST").expect("❌ 必须设置环境变量 GRPC_LOGISTIC_HOST"));

pub static GRPC_WMS_HOST: Lazy<String> =
    Lazy::new(|| env::var("GRPC_WMS_HOST").expect("❌ 必须设置环境变量 GRPC_WMS_HOST"));

pub static GRPC_PROCUREMENT_HOST: Lazy<String> = Lazy::new(|| {
    env::var("GRPC_PROCUREMENT_HOST").expect("❌ 必须设置环境变量 GRPC_PROCUREMENT_HOST")
});
