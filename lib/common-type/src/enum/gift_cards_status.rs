use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DeriveActiveEnum, EnumIter, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "gift_cards_status")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GiftCardsStatus {
    /// 未使用
    #[sea_orm(string_value = "UNUSED")]
    Unused,

    /// 已使用
    #[sea_orm(string_value = "USED")]
    Used,

    /// 已作废
    #[sea_orm(string_value = "VOIDED")]
    Voided,
}
