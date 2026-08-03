use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DeriveActiveEnum, EnumIter, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "inbound_order_type")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InboundOrderType {
    /// 采购入库
    #[sea_orm(string_value = "PURCHASE")]
    Purchase,

    /// 退货入库
    #[sea_orm(string_value = "CUSTOMER_RETURN")]
    CustomerReturn,

    /// 调拨入库
    #[sea_orm(string_value = "TRANSFER")]
    Transfer,

    /// 生产入库
    #[sea_orm(string_value = "PRODUCTION")]
    Production,

    /// 报溢入库
    #[sea_orm(string_value = "INVENTORY_SURPLUS")]
    InventorySurplus,

    /// 报损入库
    #[sea_orm(string_value = "DAMAGE_RETURN")]
    DamageReturn,

    /// 其他入库
    #[sea_orm(string_value = "OTHER")]
    Other,
}
