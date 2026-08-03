use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DeriveActiveEnum, EnumIter, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "task_status")]
#[serde(rename_all = "UPPERCASE")]
pub enum TaskStatus {
    /// 待处理
    #[sea_orm(string_value = "PENDING")]
    Pending,

    /// 已完成
    #[sea_orm(string_value = "COMPLETED")]
    Completed,

    /// 已撤销
    #[sea_orm(string_value = "CANCELLED")]
    Cancelled,
}
