use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DeriveActiveEnum, EnumIter, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "logistics_status")]
#[serde(rename_all = "UPPERCASE")]
pub enum LogisticsStatus {
    /// 待处理
    #[sea_orm(string_value = "PENDING")]
    Pending,

    /// 已分配配送资源
    #[sea_orm(string_value = "ALLOCATED")]
    Allocated,

    /// 拣货中
    #[sea_orm(string_value = "PICKING")]
    Picking,

    /// 已打包
    #[sea_orm(string_value = "PACKED")]
    Packed,

    /// 配送中
    #[sea_orm(string_value = "DELIVERING")]
    Delivering,

    /// 已送达
    #[sea_orm(string_value = "DELIVERED")]
    Delivered,

    /// 已签收
    #[sea_orm(string_value = "SIGNED")]
    Signed,
}
