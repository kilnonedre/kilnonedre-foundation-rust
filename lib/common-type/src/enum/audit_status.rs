use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DeriveActiveEnum, EnumIter, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "audit_status")]
#[serde(rename_all = "UPPERCASE")]
pub enum AuditStatus {
    /// 创建
    #[sea_orm(string_value = "CREATE")]
    Create,

    /// 更新
    #[sea_orm(string_value = "UPDATE")]
    Update,

    /// 删除
    #[sea_orm(string_value = "DELETE")]
    Delete,
}
