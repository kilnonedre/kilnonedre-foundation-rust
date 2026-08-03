use crate::{
    snapshot::catalog_snap::product_spu_tag_snap::CatalogProductSpuTagSnap,
    CatalogAggregateProductSpuTagModel, CatalogProductSpuTagModel,
};

pub fn aggregate_to_snap(model: &CatalogAggregateProductSpuTagModel) -> CatalogProductSpuTagSnap {
    CatalogProductSpuTagSnap {
        id: model.id,
        name: model.name.clone(),
    }
}

pub fn model_to_snap(model: &CatalogProductSpuTagModel) -> CatalogProductSpuTagSnap {
    CatalogProductSpuTagSnap {
        id: model.id,
        name: model.name.clone(),
    }
}
