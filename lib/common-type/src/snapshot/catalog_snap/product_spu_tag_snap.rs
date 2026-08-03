use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// 商品快照
#[derive(Serialize, ToSchema, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogProductSpuTagSnap {
    /// ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 名称
    #[schema(example = "xxxx")]
    pub name: String,
}
