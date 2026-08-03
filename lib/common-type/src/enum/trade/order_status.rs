use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DeriveActiveEnum, EnumIter, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "order_status")]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderStatus {
    /// 已创建
    #[sea_orm(string_value = "CREATED")]
    Created,

    /// 已确认
    #[sea_orm(string_value = "CONFIRMED")]
    Confirmed,

    /// 已完成
    #[sea_orm(string_value = "COMPLETED")]
    Completed,

    /// 已取消
    #[sea_orm(string_value = "CANCELLED")]
    Cancelled,

    /// 已关闭
    #[sea_orm(string_value = "CLOSED")]
    Closed,
}
