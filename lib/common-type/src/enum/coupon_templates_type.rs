use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DeriveActiveEnum, EnumIter, ToSchema,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "coupon_templates_type"
)]
#[serde(rename_all = "UPPERCASE")]
pub enum CouponTemplatesType {
    /// 满减券
    #[sea_orm(string_value = "FULL_REDUCTION")]
    FullReduction,

    /// 折扣券
    #[sea_orm(string_value = "DISCOUNT")]
    Discount,
}
