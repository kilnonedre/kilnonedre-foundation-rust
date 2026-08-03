use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    snapshot::{
        catalog_snap::{
            product_snap::CatalogProductSnap,
            product_spu_category_snap::CatalogProductSpuCategorySnap,
            product_spu_tag_snap::CatalogProductSpuTagSnap,
        },
        procurement_snap::{
            purchaser_snap::ProcurementPurchaserSnap, supplier_snap::ProcurementSupplierSnap,
        },
    },
    GeoLocationResp, ProcurementType, StorageMethod,
};

/// 商品快照
#[derive(Serialize, ToSchema, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogProductSpuSnap {
    /// ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 商品
    pub product: CatalogProductSnap,

    /// 名称
    #[schema(example = "xxxx")]
    pub name: String,

    /// 编码
    #[schema(example = "xxxx")]
    pub code: String,

    /// 分类
    pub categories: Vec<CatalogProductSpuCategorySnap>,

    /// 标签
    pub tags: Option<Vec<CatalogProductSpuTagSnap>>,

    /// 采购类型
    #[schema(example = "Supplier")]
    pub procurement_type: ProcurementType,

    /// 采购员
    pub purchaser: Option<ProcurementPurchaserSnap>,

    /// 供应商
    pub supplier: Option<ProcurementSupplierSnap>,

    /// 采购负责人
    pub purchaser_manager: ProcurementPurchaserSnap,

    /// 是否为标品
    #[schema(example = false)]
    pub is_standard: bool,

    /// 地址
    pub location: Option<GeoLocationResp>,

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
