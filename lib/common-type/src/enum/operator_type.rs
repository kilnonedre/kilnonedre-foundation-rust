use std::str::FromStr;

use kilnonedre_common_grpc::{common::GrpcOperatorType, util::error::grpc_err_internal_msg};
use kilnonedre_common_web::{util::error::svc_err_internal_msg, ApiError};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use tonic::Status;
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DeriveActiveEnum, EnumIter, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "operator_type")]
#[serde(rename_all = "UPPERCASE")]
pub enum OperatorType {
    /// 管理员
    #[sea_orm(string_value = "ADMIN")]
    Admin,

    /// 商户
    #[sea_orm(string_value = "SELLER")]
    Seller,

    /// 用户
    #[sea_orm(string_value = "CONSUMER")]
    Consumer,
}

impl FromStr for OperatorType {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "ADMIN" => Ok(Self::Admin),
            "SELLER" => Ok(Self::Seller),
            "CONSUMER" => Ok(Self::Consumer),
            _ => Err(format!("非法 operator_type: {}", value)),
        }
    }
}

impl From<OperatorType> for GrpcOperatorType {
    fn from(value: OperatorType) -> Self {
        match value {
            OperatorType::Admin => Self::Admin,
            OperatorType::Seller => Self::Seller,
            OperatorType::Consumer => Self::Consumer,
        }
    }
}

impl TryFrom<GrpcOperatorType> for OperatorType {
    type Error = String;

    fn try_from(value: GrpcOperatorType) -> Result<Self, Self::Error> {
        match value {
            GrpcOperatorType::Admin => Ok(Self::Admin),
            GrpcOperatorType::Seller => Ok(Self::Seller),
            GrpcOperatorType::Consumer => Ok(Self::Consumer),
            _ => Err(format!("非法 operator_type: {:?}", value)),
        }
    }
}

impl From<OperatorType> for i32 {
    fn from(value: OperatorType) -> Self {
        let grpc_value: GrpcOperatorType = value.into();
        grpc_value.into()
    }
}

impl TryFrom<i32> for OperatorType {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let grpc_value = GrpcOperatorType::try_from(value)
            .map_err(|_| format!("非法 operator_type: {}", value))?;

        grpc_value.try_into()
    }
}

pub fn svc_to_operator_type(value: i32) -> Result<OperatorType, ApiError> {
    OperatorType::try_from(value).map_err(|e| svc_err_internal_msg(&e))
}

pub fn grpc_to_operator_type(value: i32) -> Result<OperatorType, Status> {
    OperatorType::try_from(value).map_err(|e| grpc_err_internal_msg(&e))
}

pub fn svc_to_grpc_operator_type(value: i32) -> Result<GrpcOperatorType, ApiError> {
    GrpcOperatorType::try_from(value)
        .map_err(|_| svc_err_internal_msg(&format!("非法 operator_type: {}", value)))
}

pub fn grpc_to_grpc_operator_type(value: i32) -> Result<GrpcOperatorType, Status> {
    GrpcOperatorType::try_from(value)
        .map_err(|_| grpc_err_internal_msg(&format!("非法 operator_type: {}", value)))
}
