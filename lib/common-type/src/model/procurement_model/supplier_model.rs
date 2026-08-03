use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::CrmAccountModel;

#[derive(Debug, Serialize, ToSchema, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcurementSupplierModel {
    /// 供应商 ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 名称
    #[schema(example = "杭州鲜达食品有限公司")]
    pub name: String,

    /// 编码
    #[schema(example = "SUP0001")]
    pub code: Option<String>,

    /// 商户
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub seller_profile_id: Uuid,

    /// 地址
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub location_id: Uuid,

    /// 详细地址
    #[schema(example = "教学楼后厨收货口")]
    pub location_detail: Option<String>,

    /// 供应商资质文件 ID 列表
    #[schema(example = json!(["xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"]))]
    pub certificate_ids: Vec<Uuid>,
}

#[derive(Debug, Serialize, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProcurementAggregateSupplierModel {
    /// 供应商 ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 名称
    #[schema(example = "杭州鲜达食品有限公司")]
    pub name: String,

    /// 编码
    #[schema(example = "SUP0001")]
    pub code: Option<String>,

    /// 商户
    pub seller_profile: CrmAccountModel,

    /// 地址
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub location_id: Uuid,

    /// 详细地址
    #[schema(example = "教学楼后厨收货口")]
    pub location_detail: Option<String>,

    /// 供应商资质文件 ID 列表
    #[schema(example = json!(["xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"]))]
    pub certificate_ids: Vec<Uuid>,
}
