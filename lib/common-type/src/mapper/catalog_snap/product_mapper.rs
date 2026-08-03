use crate::{
    snapshot::catalog_snap::product_snap::CatalogProductSnap, CatalogAggregateProductModel,
    CatalogProductModel,
};

pub fn aggregate_to_snap(model: &CatalogAggregateProductModel) -> CatalogProductSnap {
    CatalogProductSnap {
        id: model.id,
        name: model.name.clone(),
        code: model.code.clone(),
        alias: model.alias.clone(),
        remark: model.remark.clone(),
    }
}

pub fn model_to_snap(model: &CatalogProductModel) -> CatalogProductSnap {
    CatalogProductSnap {
        id: model.id,
        name: model.name.clone(),
        code: model.code.clone(),
        alias: model.alias.clone(),
        remark: model.remark.clone(),
    }
}
