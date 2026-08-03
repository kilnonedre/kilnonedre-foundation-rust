use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// 车辆快照
#[derive(Serialize, ToSchema, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogisticsCarSnap {
    /// ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 车牌号
    #[schema(example = "xxxx")]
    pub plate_no: String,

    /// 车辆载重
    #[schema(example = "1.50")]
    pub capacity_weight: Decimal,

    /// 车辆容积
    #[schema(example = "1.50")]
    pub capacity_volume: Decimal,
}
