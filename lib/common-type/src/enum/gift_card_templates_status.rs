use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DeriveActiveEnum, EnumIter, ToSchema,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "gift_card_templates_status"
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GiftCardTemplatesStatus {
    /// 草稿
    #[sea_orm(string_value = "DRAFT")]
    Draft,

    /// 已发布
    #[sea_orm(string_value = "PUBLISHED")]
    Published,

    /// 已停用
    #[sea_orm(string_value = "DISABLED")]
    Disabled,
}
