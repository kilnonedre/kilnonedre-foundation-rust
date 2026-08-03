use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DeriveActiveEnum, EnumIter, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "assignee_type")]
#[serde(rename_all = "UPPERCASE")]
pub enum AssigneeType {
    /// 人物
    #[sea_orm(string_value = "USER")]
    User,

    /// 角色
    #[sea_orm(string_value = "ROLE")]
    Role,

    /// 系统
    #[sea_orm(string_value = "SYSTEM")]
    System,
}
