use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// 商品快照
#[derive(Serialize, ToSchema, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogProductSnap {
    /// ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 名称
    #[schema(example = "xxxx")]
    pub name: String,

    /// 编码
    #[schema(example = "xxxx")]
    pub code: String,

    /// 别名
    #[schema(example = "xxxx")]
    pub alias: Option<String>,

    /// 备注
    #[schema(example = "每周一三五配送")]
    pub remark: Option<String>,
}
