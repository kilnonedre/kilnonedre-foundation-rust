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
}
