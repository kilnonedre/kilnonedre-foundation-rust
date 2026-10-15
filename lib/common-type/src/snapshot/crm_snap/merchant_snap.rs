use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema, Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrmMerchantSnap {
    /// 唯一标识符
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 名称
    #[schema(example = "xxxxxxxxxx")]
    pub name: String,

    /// 编码
    #[schema(example = "xxxxxxxxxx")]
    pub code: String,

    /// 证件号码
    #[schema(example = "xxxxxxxx")]
    pub license_no: Option<String>,

    /// Logo ID
    #[schema(example = "xxxxxxxx")]
    pub logo_id: Option<Uuid>,
}
