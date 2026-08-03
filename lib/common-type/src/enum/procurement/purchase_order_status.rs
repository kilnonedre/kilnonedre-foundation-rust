use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DeriveActiveEnum, EnumIter, ToSchema,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "purchase_order_status"
)]
#[serde(rename_all = "UPPERCASE")]
pub enum PurchaseOrderStatus {
    /// 待采购
    #[sea_orm(string_value = "PENDING")]
    Pending,

    /// 采购中
    #[sea_orm(string_value = "PURCHASING")]
    Purchasing,

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
