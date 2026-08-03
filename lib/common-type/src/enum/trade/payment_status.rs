use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DeriveActiveEnum, EnumIter, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "payment_status")]
#[serde(rename_all = "UPPERCASE")]
pub enum PaymentStatus {
    /// 未支付
    #[sea_orm(string_value = "UNPAID")]
    Unpaid,

    /// 部分支付
    #[sea_orm(string_value = "PARTIAL_PAID")]
    PartialPaid,

    /// 已支付
    #[sea_orm(string_value = "PAID")]
    Paid,

    /// 部分退款中
    #[sea_orm(string_value = "PARTIAL_REFUNDING")]
    PartialRefunding,

    /// 部分已退款
    #[sea_orm(string_value = "PARTIAL_REFUNDED")]
    PartialRefunded,

    /// 全额退款中
    #[sea_orm(string_value = "REFUNDING")]
    Refunding,

    /// 已全额退款
    #[sea_orm(string_value = "REFUNDED")]
    Refunded,
}
