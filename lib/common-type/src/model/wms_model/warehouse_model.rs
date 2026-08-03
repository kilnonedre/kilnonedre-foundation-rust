use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::GeoLocationResp;

#[derive(Debug, Serialize, ToSchema, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WmsWarehouseModel {
    /// 仓库 ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 名称
    #[schema(example = "xxxxxxxx")]
    pub name: String,

    /// 编码
    #[schema(example = "xxxxxxxx")]
    pub code: Option<String>,

    /// 是否启用
    #[schema(example = false)]
    pub is_enabled: bool,

    /// 定位 ID
    #[schema(example = false)]
    pub location_id: Uuid,

    /// 详细地址
    #[schema(example = "xxxxxxxx")]
    pub location_detail: Option<String>,
}

#[derive(Debug, Serialize, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WmsAggregateWarehouseModel {
    /// 仓库 ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 名称
    #[schema(example = "xxxxxxxx")]
    pub name: String,

    /// 编码
    #[schema(example = "xxxxxxxx")]
    pub code: Option<String>,

    /// 是否启用
    #[schema(example = false)]
    pub is_enabled: bool,

    /// 定位
    pub location: GeoLocationResp,

    /// 详细地址
    #[schema(example = "xxxxxxxx")]
    pub location_detail: Option<String>,
}
