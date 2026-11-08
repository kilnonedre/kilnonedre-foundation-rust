use std::str::FromStr;

use kilnonedre_common_grpc::{common::GrpcGenderType, util::error::grpc_err_internal_msg};
use kilnonedre_common_web::{util::error::svc_err_internal_msg, ApiError};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use tonic::Status;
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DeriveActiveEnum, EnumIter, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "gender_type")]
#[serde(rename_all = "UPPERCASE")]
pub enum GenderType {
    /// 男
    #[sea_orm(string_value = "MALE")]
    Male,

    /// 女
    #[sea_orm(string_value = "FEMALE")]
    Female,

    /// 未知
    #[sea_orm(string_value = "UNKNOWN")]
    Unknown,
}

impl FromStr for GenderType {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "MALE" => Ok(Self::Male),
            "FEMALE" => Ok(Self::Female),
            "UNKNOWN" => Ok(Self::Unknown),
            _ => Err(format!("非法 gender_type: {}", value)),
        }
    }
}

impl From<GenderType> for GrpcGenderType {
    fn from(value: GenderType) -> Self {
        match value {
            GenderType::Male => Self::Male,
            GenderType::Female => Self::Female,
            GenderType::Unknown => Self::Unknown,
        }
    }
}

impl TryFrom<GrpcGenderType> for GenderType {
    type Error = String;

    fn try_from(value: GrpcGenderType) -> Result<Self, Self::Error> {
        match value {
            GrpcGenderType::Male => Ok(Self::Male),
            GrpcGenderType::Female => Ok(Self::Female),
            GrpcGenderType::Unknown => Ok(Self::Unknown),
            _ => Err(format!("非法 gender_type: {:?}", value)),
        }
    }
}

impl From<GenderType> for i32 {
    fn from(value: GenderType) -> Self {
        let grpc_value: GrpcGenderType = value.into();
        grpc_value.into()
    }
}

impl TryFrom<i32> for GenderType {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let grpc_value =
            GrpcGenderType::try_from(value).map_err(|_| format!("非法 gender_type: {}", value))?;

        grpc_value.try_into()
    }
}

pub fn svc_to_gender_type(value: i32) -> Result<GenderType, ApiError> {
    GenderType::try_from(value).map_err(|e| svc_err_internal_msg(&e))
}

pub fn grpc_to_gender_type(value: i32) -> Result<GenderType, Status> {
    GenderType::try_from(value).map_err(|e| grpc_err_internal_msg(&e))
}

pub fn svc_to_grpc_gender_type(value: i32) -> Result<GrpcGenderType, ApiError> {
    GrpcGenderType::try_from(value)
        .map_err(|_| svc_err_internal_msg(&format!("非法 gender_type: {}", value)))
}

pub fn grpc_to_grpc_gender_type(value: i32) -> Result<GrpcGenderType, Status> {
    GrpcGenderType::try_from(value)
        .map_err(|_| grpc_err_internal_msg(&format!("非法 gender_type: {}", value)))
}
