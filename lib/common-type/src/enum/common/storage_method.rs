use std::str::FromStr;

use common_grpc::{catalog::GrpcCatalogStorageMethod, util::error::grpc_err_internal_msg};
use common_web::{util::error::svc_err_internal_msg, ApiError};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use tonic::Status;
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DeriveActiveEnum, EnumIter, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "storage_method")]
#[serde(rename_all = "UPPERCASE")]
pub enum StorageMethod {
    /// 冷藏
    #[sea_orm(string_value = "COLD")]
    Cold,

    /// 冷冻
    #[sea_orm(string_value = "FROZEN")]
    Frozen,

    /// 常温
    #[sea_orm(string_value = "NORMAL")]
    Normal,
}

impl FromStr for StorageMethod {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "COLD" => Ok(Self::Cold),
            "FROZEN" => Ok(Self::Frozen),
            "NORMAL" => Ok(Self::Normal),
            _ => Err(format!("非法 storage_method: {value}")),
        }
    }
}

impl From<StorageMethod> for GrpcCatalogStorageMethod {
    fn from(value: StorageMethod) -> Self {
        match value {
            StorageMethod::Cold => Self::Cold,
            StorageMethod::Frozen => Self::Frozen,
            StorageMethod::Normal => Self::Normal,
        }
    }
}

impl TryFrom<GrpcCatalogStorageMethod> for StorageMethod {
    type Error = String;

    fn try_from(value: GrpcCatalogStorageMethod) -> Result<Self, Self::Error> {
        match value {
            GrpcCatalogStorageMethod::Cold => Ok(Self::Cold),
            GrpcCatalogStorageMethod::Frozen => Ok(Self::Frozen),
            GrpcCatalogStorageMethod::Normal => Ok(Self::Normal),
            GrpcCatalogStorageMethod::Unspecified => Err("storage_method 未指定".to_string()),
        }
    }
}

impl From<StorageMethod> for i32 {
    fn from(value: StorageMethod) -> Self {
        GrpcCatalogStorageMethod::from(value) as i32
    }
}

impl TryFrom<i32> for StorageMethod {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let grpc_value = GrpcCatalogStorageMethod::try_from(value)
            .map_err(|_| format!("非法 storage_method: {value}"))?;

        grpc_value.try_into()
    }
}

pub fn svc_to_storage_method(value: i32) -> Result<StorageMethod, ApiError> {
    StorageMethod::try_from(value).map_err(|e| svc_err_internal_msg(&e))
}

pub fn grpc_to_storage_method(value: i32) -> Result<StorageMethod, Status> {
    StorageMethod::try_from(value).map_err(|e| grpc_err_internal_msg(&e))
}

pub fn svc_to_grpc_storage_method(value: i32) -> Result<GrpcCatalogStorageMethod, ApiError> {
    GrpcCatalogStorageMethod::try_from(value)
        .map_err(|_| svc_err_internal_msg(&format!("非法 storage_method: {}", value)))
}

pub fn grpc_to_grpc_storage_method(value: i32) -> Result<GrpcCatalogStorageMethod, Status> {
    GrpcCatalogStorageMethod::try_from(value)
        .map_err(|_| grpc_err_internal_msg(&format!("非法 storage_method: {}", value)))
}
