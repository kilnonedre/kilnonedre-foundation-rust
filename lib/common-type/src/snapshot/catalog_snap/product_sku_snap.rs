use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::snapshot::catalog_snap::{
    product_sku_organization_type_price_snap::CatalogProductSkuOrganizationTypePriceSnap,
    product_sku_unit_snap::CatalogProductSkuUnitSnap, product_spu_snap::CatalogProductSpuSnap,
};

/// 商品快照
#[derive(Serialize, ToSchema, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogProductSkuSnap {
    /// ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 商品变体
    pub product_spu: CatalogProductSpuSnap,

    /// 商品变体 Sku 单位
    pub product_sku_unit: CatalogProductSkuUnitSnap,

    /// 单位数量
    #[schema(example = "20")]
    pub unit_quantity: i32,

    /// 是否为基础单位
    #[schema(example = false)]
    pub is_base_unit: bool,

    /// 基础 Sku
    #[schema(no_recursion)]
    pub base_product_sku: Option<Box<CatalogProductSkuSnap>>,

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
    pub organization_type_price: Option<CatalogProductSkuOrganizationTypePriceSnap>,
}
