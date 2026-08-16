use once_cell::sync::Lazy;

use crate::util::load_env_u16;

pub static GRPC_AUTH_PORT: Lazy<u16> = Lazy::new(|| load_env_u16("GRPC_AUTH_PORT"));

pub static GRPC_CRM_PORT: Lazy<u16> = Lazy::new(|| load_env_u16("GRPC_CRM_PORT"));

pub static GRPC_STORAGE_PORT: Lazy<u16> = Lazy::new(|| load_env_u16("GRPC_STORAGE_PORT"));

pub static GRPC_WORKFLOW_PORT: Lazy<u16> = Lazy::new(|| load_env_u16("GRPC_WORKFLOW_PORT"));

pub static GRPC_PAYMENT_PORT: Lazy<u16> = Lazy::new(|| load_env_u16("GRPC_PAYMENT_PORT"));

pub static GRPC_NOTIFICATION_PORT: Lazy<u16> = Lazy::new(|| load_env_u16("GRPC_NOTIFICATION_PORT"));

pub static GRPC_GEO_PORT: Lazy<u16> = Lazy::new(|| load_env_u16("GRPC_GEO_PORT"));
