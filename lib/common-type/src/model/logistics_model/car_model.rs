use rust_decimal::Decimal;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LogisticsCarModel {
    /// ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 车牌号
    #[schema(example = "xxxx")]
    pub plate_no: String,

    /// 车辆载重
    #[schema(example = "1.5")]
    pub capacity_weight: Decimal,

    /// 车辆容积
    #[schema(example = "1.5")]
    pub capacity_volume: Decimal,
}

#[derive(Debug, Serialize, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LogisticsAggregateCarModel {
    /// ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 车牌号
    #[schema(example = "xxxx")]
    pub plate_no: String,

    /// 车辆载重
    #[schema(example = "1.5")]
    pub capacity_weight: Decimal,

    /// 车辆容积
    #[schema(example = "1.5")]
    pub capacity_volume: Decimal,
}
