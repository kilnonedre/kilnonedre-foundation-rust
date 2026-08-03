use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DeriveActiveEnum, EnumIter, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "warehouse_bin_type")]
#[serde(rename_all = "UPPERCASE")]
pub enum WarehouseBinType {
    /// 库区
    #[sea_orm(string_value = "AREA")]
    Area,

    /// 货架
    #[sea_orm(string_value = "RACK")]
    Rack,

    /// 库位
    #[sea_orm(string_value = "BIN")]
    Bin,
}
