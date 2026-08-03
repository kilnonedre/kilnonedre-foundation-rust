use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DeriveActiveEnum, EnumIter, ToSchema,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "wallet_record_biz_type"
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WalletRecordBizType {
    /// 钱包充值
    #[sea_orm(string_value = "RECHARGE")]
    Recharge,

    /// 订单消费
    #[sea_orm(string_value = "ORDER_CONSUME")]
    OrderConsume,

    /// 订单退款
    #[sea_orm(string_value = "ORDER_REFUND")]
    OrderRefund,

    /// 管理员调整
    #[sea_orm(string_value = "ADMIN_ADJUST")]
    AdminAdjust,

    /// 余额过期
    #[sea_orm(string_value = "EXPIRED")]
    Expired,
}
