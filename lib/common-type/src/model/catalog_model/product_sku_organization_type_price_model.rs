use rust_decimal::Decimal;
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogProductSkuOrganizationTypePriceModel {
    /// ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 商品变体Sku ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub product_sku_id: Uuid,

    /// 组织客户类型
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub organization_type_id: Uuid,

    /// 价格
    #[schema(example = 2.3)]
    pub price: Decimal,

    /// 排序
    #[schema(example = 1)]
    pub sort: i32,
}

#[derive(Debug, Serialize, ToSchema, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogAggregateProductSkuOrganizationTypePriceModel {
    /// ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 商品变体Sku ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub product_sku_id: Uuid,

    /// 组织客户类型
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub organization_type_id: Uuid,

    /// 价格
    #[schema(example = 2.3)]
    pub price: Decimal,

    /// 排序
    #[schema(example = 1)]
    pub sort: i32,
}
