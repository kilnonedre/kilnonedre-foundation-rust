use crate::{
    snapshot::catalog_snap::product_sku_organization_type_price_snap::CatalogProductSkuOrganizationTypePriceSnap,
    CatalogAggregateProductSkuOrganizationTypePriceModel,
    CatalogProductSkuOrganizationTypePriceModel,
};

pub fn aggregate_to_snap(
    model: &CatalogAggregateProductSkuOrganizationTypePriceModel,
) -> CatalogProductSkuOrganizationTypePriceSnap {
    CatalogProductSkuOrganizationTypePriceSnap {
        id: model.id,
        product_sku_id: model.product_sku_id,
        organization_type_id: model.organization_type_id,
        price: model.price,
        sort: model.sort,
    }
}

pub fn model_to_snap(
    model: &CatalogProductSkuOrganizationTypePriceModel,
) -> CatalogProductSkuOrganizationTypePriceSnap {
    CatalogProductSkuOrganizationTypePriceSnap {
        id: model.id,
        product_sku_id: model.product_sku_id,
        organization_type_id: model.organization_type_id,
        price: model.price,
        sort: model.sort,
    }
}
