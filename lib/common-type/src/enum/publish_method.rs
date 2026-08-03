use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DeriveActiveEnum, EnumIter, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "publish_method")]
#[serde(rename_all = "UPPERCASE")]
pub enum PublishMethod {
    /// 立刻上线
    #[sea_orm(string_value = "IMMEDIATE")]
    Immediate,

    /// 定时
    #[sea_orm(string_value = "SCHEDULED")]
    Scheduled,

    /// 入库
    #[sea_orm(string_value = "WAREHOUSE")]
    Warehouse,
}
