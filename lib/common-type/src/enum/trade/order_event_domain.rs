use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DeriveActiveEnum, EnumIter, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "order_event_domain")]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderEventDomain {
    /// 订单领域
    #[sea_orm(string_value = "ORDER")]
    Order,

    /// 支付领域
    #[sea_orm(string_value = "PAYMENT")]
    Payment,

    /// 物流领域
    #[sea_orm(string_value = "LOGISTICS")]
    Logistics,
}
