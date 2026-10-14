use std::str::FromStr;

use kilnonedre_common_grpc::{common::GrpcApprovalStatus, util::error::grpc_err_internal_msg};
use kilnonedre_common_web::{util::error::svc_err_internal_msg, ApiError};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use tonic::Status;
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DeriveActiveEnum, EnumIter, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "approval_status")]
#[serde(rename_all = "UPPERCASE")]
pub enum ApprovalStatus {
    /// 待审核
    #[sea_orm(string_value = "PENDING")]
    Pending,

    /// 审核通过
    #[sea_orm(string_value = "APPROVED")]
    Approved,

    /// 审核拒绝
    #[sea_orm(string_value = "REJECTED")]
    Rejected,

    /// 用户取消
    #[sea_orm(string_value = "CANCELLED")]
    Cancelled,
}

impl FromStr for ApprovalStatus {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "PENDING" => Ok(Self::Pending),
            "APPROVED" => Ok(Self::Approved),
            "REJECTED" => Ok(Self::Rejected),
            "CANCELLED" => Ok(Self::Cancelled),
            _ => Err(format!("非法 approval_status: {}", value)),
        }
    }
}

impl From<ApprovalStatus> for GrpcApprovalStatus {
    fn from(value: ApprovalStatus) -> Self {
        match value {
            ApprovalStatus::Pending => Self::Pending,
            ApprovalStatus::Approved => Self::Approved,
            ApprovalStatus::Rejected => Self::Rejected,
            ApprovalStatus::Cancelled => Self::Cancelled,
        }
    }
}

impl TryFrom<GrpcApprovalStatus> for ApprovalStatus {
    type Error = String;

    fn try_from(value: GrpcApprovalStatus) -> Result<Self, Self::Error> {
        match value {
            GrpcApprovalStatus::Pending => Ok(Self::Pending),
            GrpcApprovalStatus::Approved => Ok(Self::Approved),
            GrpcApprovalStatus::Rejected => Ok(Self::Rejected),
            GrpcApprovalStatus::Cancelled => Ok(Self::Cancelled),
            _ => Err(format!("非法 approval_status: {:?}", value)),
        }
    }
}

impl From<ApprovalStatus> for i32 {
    fn from(value: ApprovalStatus) -> Self {
        let grpc_value: GrpcApprovalStatus = value.into();
        grpc_value.into()
    }
}

impl TryFrom<i32> for ApprovalStatus {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let grpc_value = GrpcApprovalStatus::try_from(value)
            .map_err(|_| format!("非法 approval_status: {}", value))?;

        grpc_value.try_into()
    }
}

pub fn svc_to_approval_status(value: i32) -> Result<ApprovalStatus, ApiError> {
    ApprovalStatus::try_from(value).map_err(|e| svc_err_internal_msg(&e))
}

pub fn grpc_to_approval_status(value: i32) -> Result<ApprovalStatus, Status> {
    ApprovalStatus::try_from(value).map_err(|e| grpc_err_internal_msg(&e))
}

pub fn svc_to_grpc_approval_status(value: i32) -> Result<GrpcApprovalStatus, ApiError> {
    GrpcApprovalStatus::try_from(value)
        .map_err(|_| svc_err_internal_msg(&format!("非法 approval_status: {}", value)))
}

pub fn grpc_to_grpc_approval_status(value: i32) -> Result<GrpcApprovalStatus, Status> {
    GrpcApprovalStatus::try_from(value)
        .map_err(|_| grpc_err_internal_msg(&format!("非法 approval_status: {}", value)))
}
