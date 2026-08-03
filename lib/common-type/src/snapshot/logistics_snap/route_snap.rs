use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::snapshot::logistics_snap::{
    area_snap::LogisticsAreaSnap, driver_snap::LogisticsDriverSnap,
};

/// 路线快照
#[derive(Serialize, ToSchema, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogisticsRouteSnap {
    /// ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 区域
    pub area: LogisticsAreaSnap,

    /// 颜色
    #[schema(example = "#000000")]
    pub color: Option<String>,

    /// 备注
    #[schema(example = "负责东城区所有门店配送")]
    pub remark: Option<String>,

    /// 司机
    pub driver: LogisticsDriverSnap,
}
