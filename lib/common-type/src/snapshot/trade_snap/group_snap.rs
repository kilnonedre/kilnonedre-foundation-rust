use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::GeoLocationResp;

/// 集团快照
#[derive(Serialize, ToSchema, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeGroupSnap {
    /// ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 名称
    #[schema(example = "xxxx")]
    pub name: String,

    /// 位置
    pub location: Option<GeoLocationResp>,

    /// 详细地址
    #[schema(example = "xxxx")]
    pub location_detail: Option<String>,
}
