use crate::{
    snapshot::catalog_snap::product_spu_category_snap::CatalogProductSpuCategorySnap,
    CatalogAggregateProductSpuCategoryModel, CatalogProductSpuCategoryModel,
};

pub fn aggregate_to_snap(
    model: &CatalogAggregateProductSpuCategoryModel,
) -> CatalogProductSpuCategorySnap {
    CatalogProductSpuCategorySnap {
        id: model.id,
        parent_id: model.parent_id,
        name: model.name.clone(),
        level: model.level,
        sort: model.sort,
        is_leaf: model.is_leaf,
        image_id: model.image_id,
    }
}

pub fn model_to_snap(model: &CatalogProductSpuCategoryModel) -> CatalogProductSpuCategorySnap {
    CatalogProductSpuCategorySnap {
        id: model.id,
        parent_id: model.parent_id,
        name: model.name.clone(),
        level: model.level,
        sort: model.sort,
        is_leaf: model.is_leaf,
        image_id: model.image_id,
    }
}
