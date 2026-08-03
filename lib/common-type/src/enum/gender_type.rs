use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DeriveActiveEnum, EnumIter, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "gender_type")]
#[serde(rename_all = "UPPERCASE")]
pub enum GenderType {
    /// 男
    #[sea_orm(string_value = "MALE")]
    Male,

    /// 女
    #[sea_orm(string_value = "FEMALE")]
    Female,

    /// 未知
    #[sea_orm(string_value = "UNKNOWN")]
    Unknown,
}
