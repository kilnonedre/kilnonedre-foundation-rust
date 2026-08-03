use std::str::FromStr;

use common_grpc::{common::GrpcMapProvider, util::error::grpc_err_internal_msg};
use common_web::{util::error::svc_err_internal_msg, ApiError};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use tonic::Status;
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DeriveActiveEnum, EnumIter, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "map_provider")]
#[serde(rename_all = "UPPERCASE")]
pub enum MapProvider {
    /// 高德地图
    #[sea_orm(string_value = "AMAP")]
    Amap,

    /// 腾讯地图
    #[sea_orm(string_value = "TENCENT")]
    Tencent,
}

impl FromStr for MapProvider {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "AMAP" => Ok(Self::Amap),
            "TENCENT" => Ok(Self::Tencent),
            _ => Err(format!("非法 map_provider: {}", value)),
        }
    }
}

impl From<MapProvider> for GrpcMapProvider {
    fn from(value: MapProvider) -> Self {
        match value {
            MapProvider::Amap => Self::Amap,
            MapProvider::Tencent => Self::Tencent,
        }
    }
}

impl TryFrom<GrpcMapProvider> for MapProvider {
    type Error = String;

    fn try_from(value: GrpcMapProvider) -> Result<Self, Self::Error> {
        match value {
            GrpcMapProvider::Amap => Ok(Self::Amap),
            GrpcMapProvider::Tencent => Ok(Self::Tencent),
            _ => Err(format!("非法 map_provider: {:?}", value)),
        }
    }
}

impl From<MapProvider> for i32 {
    fn from(value: MapProvider) -> Self {
        let grpc_value: GrpcMapProvider = value.into();
        grpc_value.into()
    }
}

impl TryFrom<i32> for MapProvider {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let grpc_value = GrpcMapProvider::try_from(value)
            .map_err(|_| format!("非法 map_provider: {}", value))?;

        grpc_value.try_into()
    }
}

pub fn svc_to_map_provider(value: i32) -> Result<MapProvider, ApiError> {
    MapProvider::try_from(value).map_err(|e| svc_err_internal_msg(&e))
}

pub fn grpc_to_map_provider(value: i32) -> Result<MapProvider, Status> {
    MapProvider::try_from(value).map_err(|e| grpc_err_internal_msg(&e))
}

pub fn svc_to_grpc_map_provider(value: i32) -> Result<GrpcMapProvider, ApiError> {
    GrpcMapProvider::try_from(value)
        .map_err(|_| svc_err_internal_msg(&format!("非法 map_provider: {}", value)))
}

pub fn grpc_to_grpc_map_provider(value: i32) -> Result<GrpcMapProvider, Status> {
    GrpcMapProvider::try_from(value)
        .map_err(|_| grpc_err_internal_msg(&format!("非法 map_provider: {}", value)))
}
