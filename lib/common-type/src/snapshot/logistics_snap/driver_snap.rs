use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::snapshot::{
    crm_snap::account_snap::CrmAccountSnap, logistics_snap::car_snap::LogisticsCarSnap,
};

/// 司机快照
#[derive(Serialize, ToSchema, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogisticsDriverSnap {
    /// ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 员工资料
    pub seller_profile: CrmAccountSnap,

    /// 车辆
    pub car: Option<LogisticsCarSnap>,

    /// 驾驶证号码
    #[schema(example = "xxxxxxxxxx")]
    pub driving_license_no: String,

    /// 资质文件 ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub certificate_ids: Vec<Uuid>,
}
