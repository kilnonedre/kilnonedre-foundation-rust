use rust_decimal::Decimal;
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::model::catalog_model::{
    product_sku_organization_type_price_model::CatalogProductSkuOrganizationTypePriceModel,
    product_sku_unit_model::CatalogProductSkuUnitModel, product_spu_model::CatalogProductSpuModel,
};

#[derive(Debug, Serialize, ToSchema, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogProductSkuModel {
    /// ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 商品变体
    pub product_spu: CatalogProductSpuModel,

    /// 商品变体 Sku 单位
    pub product_sku_unit: CatalogProductSkuUnitModel,

    /// 单位数量
    #[schema(example = "20")]
    pub unit_quantity: i32,

    /// 是否为基础单位
    #[schema(example = false)]
    pub is_base_unit: bool,

    /// 基础 Sku
    pub base_product_sku: Option<Box<CatalogProductSkuModel>>,

    /// 市场价
    #[schema(example = 3.5)]
    pub market_price: Decimal,

    /// 售价
    #[schema(example = 3.5)]
    pub sale_price: Decimal,

    /// 是否可售卖
    #[schema(example = false)]
    pub is_saleable: bool,

    /// 排序
    #[schema(example = 1)]
    pub sort: i32,

    /// 备注
    #[schema(example = "每周一三五配送")]
    pub remark: Option<String>,

    /// 客户类型价
    pub organization_type_prices: Option<Vec<CatalogProductSkuOrganizationTypePriceModel>>,
}

#[derive(Debug, Serialize, ToSchema, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogAggregateProductSkuModel {
    /// ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 商品变体
    pub product_spu: CatalogProductSpuModel,

    /// 商品变体 Sku 单位
    pub product_sku_unit: CatalogProductSkuUnitModel,

    /// 单位数量
    #[schema(example = "20")]
    pub unit_quantity: i32,

    /// 是否为基础单位
    #[schema(example = false)]
    pub is_base_unit: bool,

    /// 基础 Sku
    pub base_product_sku: Option<Box<CatalogProductSkuModel>>,

    /// 市场价
    #[schema(example = 3.5)]
    pub market_price: Decimal,

    /// 售价
    #[schema(example = 3.5)]
    pub sale_price: Decimal,

    /// 是否可售卖
    #[schema(example = false)]
    pub is_saleable: bool,

    /// 排序
    #[schema(example = 1)]
    pub sort: i32,

    /// 备注
    #[schema(example = "每周一三五配送")]
    pub remark: Option<String>,

    /// 客户类型价
    pub organization_type_prices: Option<Vec<CatalogProductSkuOrganizationTypePriceModel>>,
}
