use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{snapshot::crm_snap::account_snap::CrmAccountSnap, GeoLocationResp};

/// 供应商快照
#[derive(Serialize, ToSchema, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcurementSupplierSnap {
    /// ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 名称
    #[schema(example = "杭州鲜达食品有限公司")]
    pub name: String,

    /// 编码
    #[schema(example = "SUP0001")]
    pub code: Option<String>,

    /// 员工资料
    pub seller_profile: CrmAccountSnap,

    /// 地址
    pub location: GeoLocationResp,

    /// 详细地址
    #[schema(example = "教学楼后厨收货口")]
    pub location_detail: Option<String>,

    /// 供应商资质文件 ID 列表
    #[schema(example = json!(["xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"]))]
    pub certificate_ids: Vec<Uuid>,
}
