use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::MapProvider;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GeoLocationSnap {
    /// ID
    pub id: Uuid,

    /// 省
    pub province: Option<String>,

    /// 市
    pub city: Option<String>,

    /// 区
    pub district: Option<String>,

    /// 行政区划代码
    pub ad_code: Option<String>,

    /// 标准地址
    pub address: String,

    /// 纬度
    pub latitude: f64,

    /// 经度
    pub longitude: f64,

    /// 地图供应商 POI ID
    pub poi_id: Option<String>,

    /// POI 名称
    pub poi_name: Option<String>,

    /// 地图数据来源
    pub map_provider: MapProvider,
}
