use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    model::catalog_model::{
        product_model::CatalogProductModel,
        product_spu_category_model::CatalogProductSpuCategoryModel,
        product_spu_tag_model::CatalogProductSpuTagModel,
    },
    ProcurementPurchaserModel, ProcurementSupplierModel, ProcurementType, StorageMethod,
};

#[derive(Debug, Serialize, ToSchema, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogProductSpuModel {
    /// ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 商品
    pub product: CatalogProductModel,

    /// 名称
    #[schema(example = "xxxx")]
    pub name: String,

    /// 编码
    #[schema(example = "xxxx")]
    pub code: String,

    /// 分类
    pub categories: Vec<CatalogProductSpuCategoryModel>,

    /// 标签
    pub tags: Option<Vec<CatalogProductSpuTagModel>>,

    /// 采购类型
    #[schema(example = "Supplier")]
    pub procurement_type: ProcurementType,

    /// 采购员
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub purchaser_id: Option<Uuid>,

    /// 供应商
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub supplier_id: Option<Uuid>,

    /// 采购负责人
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub purchaser_manager_id: Uuid,

    /// 是否为标品
    #[schema(example = false)]
    pub is_standard: bool,

    /// 地址
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub location_id: Option<Uuid>,

    /// 详细地址
    #[schema(example = "华南区域连锁餐饮客户")]
    pub location_detail: Option<String>,

    /// 储藏方式
    #[schema(example = "Cold")]
    pub storage_method: StorageMethod,

    /// 封面 ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub cover_ids: Option<Vec<Uuid>>,

    /// 详情图 ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub detail_image_ids: Option<Vec<Uuid>>,

    /// 备注
    #[schema(example = "每周一三五配送")]
    pub remark: Option<String>,
}

#[derive(Debug, Serialize, ToSchema, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogAggregateProductSpuModel {
    /// ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 商品
    pub product: CatalogProductModel,

    /// 名称
    #[schema(example = "xxxx")]
    pub name: String,

    /// 编码
    #[schema(example = "xxxx")]
    pub code: String,

    /// 分类
    pub categories: Vec<CatalogProductSpuCategoryModel>,

    /// 标签
    pub tags: Option<Vec<CatalogProductSpuTagModel>>,

    /// 采购类型
    #[schema(example = "Supplier")]
    pub procurement_type: ProcurementType,

    /// 采购员
    pub purchaser: Option<ProcurementPurchaserModel>,

    /// 供应商
    pub supplier: Option<ProcurementSupplierModel>,

    /// 采购负责人
    pub purchaser_manager: ProcurementPurchaserModel,

    /// 是否为标品
    #[schema(example = false)]
    pub is_standard: bool,

    /// 地址
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub location_id: Option<Uuid>,

    /// 详细地址
    #[schema(example = "华南区域连锁餐饮客户")]
    pub location_detail: Option<String>,

    /// 储藏方式
    #[schema(example = "Cold")]
    pub storage_method: StorageMethod,

    /// 封面 ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub cover_ids: Option<Vec<Uuid>>,

    /// 详情图 ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub detail_image_ids: Option<Vec<Uuid>>,

    /// 备注
    #[schema(example = "每周一三五配送")]
    pub remark: Option<String>,
}
