use std::str::FromStr;

use common_grpc::{catalog::GrpcCatalogProcurementType, util::error::grpc_err_internal_msg};
use common_web::{util::error::svc_err_internal_msg, ApiError};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use tonic::Status;
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DeriveActiveEnum, EnumIter, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "procurement_type")]
#[serde(rename_all = "UPPERCASE")]
pub enum ProcurementType {
    /// 供应商
    #[sea_orm(string_value = "SUPPLIER")]
    Supplier,

    /// 采购员
    #[sea_orm(string_value = "PURCHASER")]
    Purchaser,
}

impl FromStr for ProcurementType {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "COLD" => Ok(Self::Supplier),
            "FROZEN" => Ok(Self::Purchaser),
            _ => Err(format!("非法 procurement_type: {value}")),
        }
    }
}

impl From<ProcurementType> for GrpcCatalogProcurementType {
    fn from(value: ProcurementType) -> Self {
        match value {
            ProcurementType::Supplier => Self::Supplier,
            ProcurementType::Purchaser => Self::Purchaser,
        }
    }
}

impl TryFrom<GrpcCatalogProcurementType> for ProcurementType {
    type Error = String;

    fn try_from(value: GrpcCatalogProcurementType) -> Result<Self, Self::Error> {
        match value {
            GrpcCatalogProcurementType::Supplier => Ok(Self::Supplier),
            GrpcCatalogProcurementType::Purchaser => Ok(Self::Purchaser),
            GrpcCatalogProcurementType::Unspecified => Err("procurement_type 未指定".to_string()),
        }
    }
}

impl From<ProcurementType> for i32 {
    fn from(value: ProcurementType) -> Self {
        GrpcCatalogProcurementType::from(value) as i32
    }
}

impl TryFrom<i32> for ProcurementType {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let grpc_value = GrpcCatalogProcurementType::try_from(value)
            .map_err(|_| format!("非法 procurement_type: {value}"))?;

        grpc_value.try_into()
    }
}

pub fn svc_to_procurement_type(value: i32) -> Result<ProcurementType, ApiError> {
    ProcurementType::try_from(value).map_err(|e| svc_err_internal_msg(&e))
}

pub fn grpc_to_procurement_type(value: i32) -> Result<ProcurementType, Status> {
    ProcurementType::try_from(value).map_err(|e| grpc_err_internal_msg(&e))
}

pub fn svc_to_grpc_procurement_type(value: i32) -> Result<GrpcCatalogProcurementType, ApiError> {
    GrpcCatalogProcurementType::try_from(value)
        .map_err(|_| svc_err_internal_msg(&format!("非法 procurement_type: {}", value)))
}

pub fn grpc_to_grpc_procurement_type(value: i32) -> Result<GrpcCatalogProcurementType, Status> {
    GrpcCatalogProcurementType::try_from(value)
        .map_err(|_| grpc_err_internal_msg(&format!("非法 procurement_type: {}", value)))
}
