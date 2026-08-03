use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DeriveActiveEnum, EnumIter, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "entity_status")]
#[serde(rename_all = "UPPERCASE")]
pub enum EntityStatus {
    /// 正常状态
    /// 数据处于可用状态，可以被查询和使用
    #[sea_orm(string_value = "ACTIVE")]
    Active,

    /// 已删除状态（逻辑删除）
    /// 数据不会被正常查询返回，但仍保留在数据库中
    #[sea_orm(string_value = "DELETED")]
    Deleted,
}
