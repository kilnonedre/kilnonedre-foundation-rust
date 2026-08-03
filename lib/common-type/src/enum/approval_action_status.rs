use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumIter, ToSchema)]
#[serde(rename_all = "UPPERCASE")]
pub enum ApprovalActionStatus {
    /// 审核通过
    Approved,

    /// 审核拒绝
    Rejected,

    /// 用户取消
    Cancelled,
}
