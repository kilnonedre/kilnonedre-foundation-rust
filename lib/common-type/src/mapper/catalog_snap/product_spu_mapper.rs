use std::collections::HashMap;

use uuid::Uuid;

use crate::{
    lookup_optional, lookup_required,
    mapper::{
        catalog_snap::{product_mapper, product_spu_category_mapper, product_spu_tag_mapper},
        procurement_mapper::{purchaser_mapper, supplier_mapper},
    },
    snapshot::catalog_snap::product_spu_snap::CatalogProductSpuSnap,
    CatalogAggregateProductSpuModel, CatalogProductSpuModel, CrmAggregateAccountModel,
    GeoLocationResp, ProcurementAggregatePurchaserModel, ProcurementAggregateSupplierModel,
};

pub fn aggregate_to_snap(
    model: &CatalogAggregateProductSpuModel,
    account_map: &HashMap<Uuid, CrmAggregateAccountModel>,
    location_map: &HashMap<Uuid, GeoLocationResp>,
) -> CatalogProductSpuSnap {
    CatalogProductSpuSnap {
        id: model.id,
        product: product_mapper::model_to_snap(&model.product),
        name: model.name.clone(),
        code: model.code.clone(),
        categories: model
            .categories
            .iter()
            .map(product_spu_category_mapper::model_to_snap)
            .collect(),
        tags: model.tags.as_ref().map(|tags| {
            tags.iter()
                .map(product_spu_tag_mapper::model_to_snap)
                .collect()
        }),
        procurement_type: model.procurement_type,
        purchaser: model
            .purchaser
            .as_ref()
            .map(|purchaser| purchaser_mapper::model_to_snap(&purchaser, account_map)),
        supplier: model
            .supplier
            .as_ref()
            .map(|supplier| supplier_mapper::model_to_snap(&supplier, account_map, location_map)),
        purchaser_manager: purchaser_mapper::model_to_snap(&model.purchaser_manager, account_map),
        is_standard: model.is_standard,
        location: lookup_optional(&model.location_id, location_map),
        location_detail: model.location_detail.clone(),
        storage_method: model.storage_method,
        cover_ids: model.cover_ids.clone(),
        detail_image_ids: model.detail_image_ids.clone(),
        remark: model.remark.clone(),
    }
}

pub fn model_to_snap(
    model: &CatalogProductSpuModel,
    purchaser_map: &HashMap<Uuid, ProcurementAggregatePurchaserModel>,
    supplier_map: &HashMap<Uuid, ProcurementAggregateSupplierModel>,
    location_map: &HashMap<Uuid, GeoLocationResp>,
) -> CatalogProductSpuSnap {
    CatalogProductSpuSnap {
        id: model.id,
        product: product_mapper::model_to_snap(&model.product),
        name: model.name.clone(),
        code: model.code.clone(),
        categories: model
            .categories
            .iter()
            .map(product_spu_category_mapper::model_to_snap)
            .collect(),
        tags: model.tags.as_ref().map(|tags| {
            tags.iter()
                .map(product_spu_tag_mapper::model_to_snap)
                .collect()
        }),
        procurement_type: model.procurement_type,
        purchaser: model.purchaser_id.as_ref().map(|purchaser_id| {
            purchaser_mapper::aggregate_to_snap(&lookup_required(purchaser_id, purchaser_map))
        }),
        supplier: model.supplier_id.as_ref().map(|supplier_id| {
            supplier_mapper::aggregate_to_snap(
                &lookup_required(supplier_id, supplier_map),
                location_map,
            )
        }),
        purchaser_manager: purchaser_mapper::aggregate_to_snap(&lookup_required(
            &model.purchaser_manager_id,
            purchaser_map,
        )),
        is_standard: model.is_standard,
        location: lookup_optional(&model.location_id, location_map),
        location_detail: model.location_detail.clone(),
        storage_method: model.storage_method,
        cover_ids: model.cover_ids.clone(),
        detail_image_ids: model.detail_image_ids.clone(),
        remark: model.remark.clone(),
    }
}
