use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DeriveActiveEnum, EnumIter, ToSchema,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "coupon_templates_scope_type"
)]
#[serde(rename_all = "UPPERCASE")]
pub enum CouponTemplatesScopeType {
    /// 全场通用
    #[sea_orm(string_value = "ALL")]
    All,

    /// 部分商品可用
    #[sea_orm(string_value = "PARTIAL")]
    Partial,
}
