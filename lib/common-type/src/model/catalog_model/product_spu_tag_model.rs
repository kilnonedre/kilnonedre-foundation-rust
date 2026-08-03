use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogProductSpuTagModel {
    /// ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 单位名称（唯一）
    #[schema(example = "手机")]
    pub name: String,
}

#[derive(Debug, Serialize, ToSchema, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogAggregateProductSpuTagModel {
    /// ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 单位名称（唯一）
    #[schema(example = "手机")]
    pub name: String,
}
