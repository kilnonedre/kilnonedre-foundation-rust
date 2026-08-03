use core::fmt;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DeriveActiveEnum, EnumIter, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "instance_status")]
#[serde(rename_all = "UPPERCASE")]
pub enum InstanceStatus {
    /// 运行中
    #[sea_orm(string_value = "RUNNING")]
    Running,

    /// 已完成
    #[sea_orm(string_value = "COMPLETED")]
    Completed,

    /// 驳回
    #[sea_orm(string_value = "REJECTED")]
    Rejected,

    /// 撤销
    #[sea_orm(string_value = "CANCELLED")]
    Cancelled,
}

impl Display for InstanceStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            InstanceStatus::Running => "RUNNING",
            InstanceStatus::Completed => "COMPLETED",
            InstanceStatus::Rejected => "REJECTED",
            InstanceStatus::Cancelled => "CANCELLED",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for InstanceStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RUNNING" => Ok(InstanceStatus::Running),
            "COMPLETED" => Ok(InstanceStatus::Completed),
            "REJECTED" => Ok(InstanceStatus::Rejected),
            "CANCELLED" => Ok(InstanceStatus::Cancelled),
            _ => Err(format!("无效的 InstanceStatus: {}", s)),
        }
    }
}

impl InstanceStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            InstanceStatus::Running => "RUNNING",
            InstanceStatus::Completed => "COMPLETED",
            InstanceStatus::Rejected => "REJECTED",
            InstanceStatus::Cancelled => "CANCELLED",
        }
    }

    pub fn parse(s: &str) -> Result<Self, String> {
        s.parse()
    }
}
