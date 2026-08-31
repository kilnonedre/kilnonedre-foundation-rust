use sea_orm::entity::prelude::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::MapProvider;

#[derive(Debug, Serialize, ToSchema, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeoLocationReq {
    /// 定位
    pub location: GeoLocationModel,

    /// 地图服务返回的原始响应数据
    pub raw_response: Json,
}

#[derive(Debug, Serialize, ToSchema, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeoAggregateLocationModel {
    /// 定位记录 ID
    pub id: Uuid,

    #[serde(flatten)]
    pub base: GeoLocationModel,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GeoLocationModel {
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
