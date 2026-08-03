use crate::{
    snapshot::catalog_snap::product_sku_unit_snap::CatalogProductSkuUnitSnap,
    CatalogAggregateProductSkuUnitModel, CatalogProductSkuUnitModel,
};

pub fn aggregate_to_snap(model: &CatalogAggregateProductSkuUnitModel) -> CatalogProductSkuUnitSnap {
    CatalogProductSkuUnitSnap {
        id: model.id,
        name: model.name.clone(),
    }
}

pub fn model_to_snap(model: &CatalogProductSkuUnitModel) -> CatalogProductSkuUnitSnap {
    CatalogProductSkuUnitSnap {
        id: model.id,
        name: model.name.clone(),
    }
}
