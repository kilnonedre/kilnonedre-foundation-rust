use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageReq {
    /// 页码
    #[schema(example = 1)]
    pub page: u64,

    /// 单页数量
    #[schema(example = 10)]
    pub size: u64,
}
