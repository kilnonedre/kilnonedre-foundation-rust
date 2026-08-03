use std::collections::HashMap;

use uuid::Uuid;

use crate::{
    mapper::catalog_snap::{
        product_sku_organization_type_price_mapper, product_sku_unit_mapper, product_spu_mapper,
    },
    snapshot::catalog_snap::{
        product_sku_organization_type_price_snap::CatalogProductSkuOrganizationTypePriceSnap,
        product_sku_snap::CatalogProductSkuSnap,
    },
    CatalogAggregateProductSkuModel, CatalogProductSkuModel,
    CatalogProductSkuOrganizationTypePriceModel, GeoLocationResp,
    ProcurementAggregatePurchaserModel, ProcurementAggregateSupplierModel,
};

fn find_organization_type_price_snap(
    organization_type_prices: &Option<Vec<CatalogProductSkuOrganizationTypePriceModel>>,
    organization_type_id: &Option<Uuid>,
) -> Option<CatalogProductSkuOrganizationTypePriceSnap> {
    organization_type_id
        .as_ref()
        .and_then(|organization_type_id| {
            organization_type_prices
                .as_ref()
                .and_then(|organization_type_prices| {
                    organization_type_prices
                        .iter()
                        .find(|organization_type_price| {
                            organization_type_price.organization_type_id == *organization_type_id
                        })
                        .map(product_sku_organization_type_price_mapper::model_to_snap)
                })
        })
}

pub fn aggregate_to_snap(
    model: &CatalogAggregateProductSkuModel,
    organization_type_id: &Option<Uuid>,
    purchaser_map: &HashMap<Uuid, ProcurementAggregatePurchaserModel>,
    supplier_map: &HashMap<Uuid, ProcurementAggregateSupplierModel>,
    location_map: &HashMap<Uuid, GeoLocationResp>,
) -> CatalogProductSkuSnap {
    CatalogProductSkuSnap {
        id: model.id,
        product_spu: product_spu_mapper::model_to_snap(
            &model.product_spu,
            purchaser_map,
            supplier_map,
            location_map,
        ),
        product_sku_unit: product_sku_unit_mapper::model_to_snap(&model.product_sku_unit),
        unit_quantity: model.unit_quantity,
        is_base_unit: model.is_base_unit,
        base_product_sku: model.base_product_sku.as_ref().map(|base_product_sku| {
            Box::new(model_to_snap(
                base_product_sku,
                organization_type_id,
                purchaser_map,
                supplier_map,
                location_map,
            ))
        }),
        market_price: model.market_price,
        sale_price: model.sale_price,
        is_saleable: model.is_saleable,
        sort: model.sort,
        remark: model.remark.clone(),
        organization_type_price: find_organization_type_price_snap(
            &model.organization_type_prices,
            organization_type_id,
        ),
    }
}

pub fn model_to_snap(
    model: &CatalogProductSkuModel,
    organization_type_id: &Option<Uuid>,
    purchaser_map: &HashMap<Uuid, ProcurementAggregatePurchaserModel>,
    supplier_map: &HashMap<Uuid, ProcurementAggregateSupplierModel>,
    location_map: &HashMap<Uuid, GeoLocationResp>,
) -> CatalogProductSkuSnap {
    CatalogProductSkuSnap {
        id: model.id,
        product_spu: product_spu_mapper::model_to_snap(
            &model.product_spu,
            purchaser_map,
            supplier_map,
            location_map,
        ),
        product_sku_unit: product_sku_unit_mapper::model_to_snap(&model.product_sku_unit),
        unit_quantity: model.unit_quantity,
        is_base_unit: model.is_base_unit,
        base_product_sku: model.base_product_sku.as_ref().map(|base_product_sku| {
            Box::new(model_to_snap(
                base_product_sku,
                organization_type_id,
                purchaser_map,
                supplier_map,
                location_map,
            ))
        }),
        market_price: model.market_price,
        sale_price: model.sale_price,
        is_saleable: model.is_saleable,
        sort: model.sort,
        remark: model.remark.clone(),
        organization_type_price: find_organization_type_price_snap(
            &model.organization_type_prices,
            organization_type_id,
        ),
    }
}
