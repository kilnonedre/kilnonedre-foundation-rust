use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
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
