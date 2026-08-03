use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DeriveActiveEnum, EnumIter, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "task_action")]
#[serde(rename_all = "UPPERCASE")]
pub enum TaskAction {
    /// 通过
    #[sea_orm(string_value = "APPROVE")]
    Approve,

    /// 驳回
    #[sea_orm(string_value = "REJECT")]
    Reject,

    /// 撤销
    #[sea_orm(string_value = "CANCEL")]
    Cancel,

    /// 发起
    #[sea_orm(string_value = "AUTO_START")]
    AutoStart,

    /// 结束
    #[sea_orm(string_value = "AUTO_END")]
    AutoEnd,
}
