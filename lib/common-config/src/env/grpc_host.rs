use once_cell::sync::Lazy;

use crate::util::load_env_str;

pub static GRPC_AUTH_HOST: Lazy<String> = Lazy::new(|| load_env_str("GRPC_AUTH_HOST"));

pub static GRPC_CRM_HOST: Lazy<String> = Lazy::new(|| load_env_str("GRPC_CRM_HOST"));

pub static GRPC_CATALOG_HOST: Lazy<String> = Lazy::new(|| load_env_str("GRPC_CATALOG_HOST"));

pub static GRPC_STORAGE_HOST: Lazy<String> = Lazy::new(|| load_env_str("GRPC_STORAGE_HOST"));

pub static GRPC_WORKFLOW_HOST: Lazy<String> = Lazy::new(|| load_env_str("GRPC_WORKFLOW_HOST"));

pub static GRPC_TRADE_HOST: Lazy<String> = Lazy::new(|| load_env_str("GRPC_TRADE_HOST"));

pub static GRPC_PAYMENT_HOST: Lazy<String> = Lazy::new(|| load_env_str("GRPC_PAYMENT_HOST"));

pub static GRPC_NOTIFICATION_HOST: Lazy<String> =
    Lazy::new(|| load_env_str("GRPC_NOTIFICATION_HOST"));

pub static GRPC_GEO_HOST: Lazy<String> = Lazy::new(|| load_env_str("GRPC_GEO_HOST"));

pub static GRPC_LOGISTIC_HOST: Lazy<String> = Lazy::new(|| load_env_str("GRPC_LOGISTIC_HOST"));

pub static GRPC_WMS_HOST: Lazy<String> = Lazy::new(|| load_env_str("GRPC_WMS_HOST"));

pub static GRPC_PROCUREMENT_HOST: Lazy<String> =
    Lazy::new(|| load_env_str("GRPC_PROCUREMENT_HOST"));
