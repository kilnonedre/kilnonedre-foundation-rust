use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DeriveActiveEnum, EnumIter, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "delivery_method")]
#[serde(rename_all = "UPPERCASE")]
pub enum DeliveryMethod {
    /// 配送
    #[sea_orm(string_value = "DELIVERY")]
    Delivery,

    /// 自提
    #[sea_orm(string_value = "PICKUP")]
    Pickup,
}
