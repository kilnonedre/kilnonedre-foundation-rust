use sea_orm::prelude::Uuid;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CrmMerchantModel {
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

#[derive(Serialize, ToSchema, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CrmCompositeMerchantModel {
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

#[derive(Serialize, ToSchema, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CrmAggregateMerchantModel {
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
