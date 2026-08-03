use sea_orm::prelude::Uuid;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CrmRoleModel {
    /// ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 名称
    #[schema(example = "username")]
    pub name: String,

    /// 唯一标识符
    #[schema(example = "example")]
    pub code: String,
}

#[derive(Serialize, ToSchema, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CrmAggregateRoleModel {
    /// ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 名称
    #[schema(example = "username")]
    pub name: String,

    /// 唯一标识符
    #[schema(example = "example")]
    pub code: String,
}
