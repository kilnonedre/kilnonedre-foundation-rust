use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DeriveActiveEnum, EnumIter, ToSchema,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "refund_item_biz_type"
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RefundItemBizType {
    /// 支付退款
    #[sea_orm(string_value = "PAYMENT_REFUND")]
    PaymentRefund,

    /// 钱包退款
    #[sea_orm(string_value = "WALLET_REFUND")]
    WalletRefund,

    /// 礼品卡退款
    #[sea_orm(string_value = "GIFT_CARD_REFUND")]
    GiftCardRefund,
}
