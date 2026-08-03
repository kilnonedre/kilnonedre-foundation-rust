use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::model::logistics_model::{
    area_model::LogisticsAreaModel, driver_model::LogisticsDriverModel,
};

#[derive(Debug, Serialize, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LogisticsRouteModel {
    /// 线路 ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 区域
    pub area: LogisticsAreaModel,

    /// 线路名称
    #[schema(example = "东城区配送线路")]
    pub name: String,

    /// 线路颜色
    #[schema(example = "#000000")]
    pub color: Option<String>,

    /// 备注
    #[schema(example = "负责东城区所有门店配送")]
    pub remark: Option<String>,

    /// 司机
    pub driver: LogisticsDriverModel,
}

#[derive(Debug, Serialize, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LogisticsAggregateRouteModel {
    /// 线路 ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 区域
    pub area: LogisticsAreaModel,

    /// 线路名称
    #[schema(example = "东城区配送线路")]
    pub name: String,

    /// 线路颜色
    #[schema(example = "#000000")]
    pub color: Option<String>,

    /// 备注
    #[schema(example = "负责东城区所有门店配送")]
    pub remark: Option<String>,

    /// 司机
    pub driver: LogisticsDriverModel,
}
