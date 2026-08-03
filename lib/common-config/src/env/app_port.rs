use once_cell::sync::Lazy;

use crate::util::load_env_u16;

pub static APP_AUTH_PORT: Lazy<u16> = Lazy::new(|| load_env_u16("APP_AUTH_PORT"));

pub static APP_CRM_PORT: Lazy<u16> = Lazy::new(|| load_env_u16("APP_CRM_PORT"));

pub static APP_CATALOG_PORT: Lazy<u16> = Lazy::new(|| load_env_u16("APP_CATALOG_PORT"));

pub static APP_STORAGE_PORT: Lazy<u16> = Lazy::new(|| load_env_u16("APP_STORAGE_PORT"));

pub static APP_WORKFLOW_PORT: Lazy<u16> = Lazy::new(|| load_env_u16("APP_WORKFLOW_PORT"));

pub static APP_TRADE_PORT: Lazy<u16> = Lazy::new(|| load_env_u16("APP_TRADE_PORT"));

pub static APP_PAYMENT_PORT: Lazy<u16> = Lazy::new(|| load_env_u16("APP_PAYMENT_PORT"));

pub static APP_NOTIFICATION_PORT: Lazy<u16> = Lazy::new(|| load_env_u16("APP_NOTIFICATION_PORT"));

pub static APP_GEO_PORT: Lazy<u16> = Lazy::new(|| load_env_u16("APP_GEO_PORT"));

pub static APP_WMS_PORT: Lazy<u16> = Lazy::new(|| load_env_u16("APP_WMS_PORT"));

pub static APP_PROCUREMENT_PORT: Lazy<u16> = Lazy::new(|| load_env_u16("APP_PROCUREMENT_PORT"));
