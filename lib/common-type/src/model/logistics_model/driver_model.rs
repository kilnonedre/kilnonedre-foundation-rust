use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{model::logistics_model::car_model::LogisticsCarModel, CrmAccountModel};

#[derive(Debug, Serialize, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LogisticsDriverModel {
    /// ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 员工资料
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub seller_profile_id: Uuid,

    /// 车辆
    pub car: Option<LogisticsCarModel>,

    /// 驾驶证号码
    #[schema(example = "xxxxxxxxxx")]
    pub driving_license_no: String,

    /// 资质文件 ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub certificate_ids: Vec<Uuid>,
}

#[derive(Debug, Serialize, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LogisticsAggregateDriverModel {
    /// ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 员工资料
    pub seller_profile: CrmAccountModel,

    /// 车辆
    pub car: Option<LogisticsCarModel>,

    /// 驾驶证号码
    #[schema(example = "xxxxxxxxxx")]
    pub driving_license_no: String,

    /// 资质文件 ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub certificate_ids: Vec<Uuid>,
}
