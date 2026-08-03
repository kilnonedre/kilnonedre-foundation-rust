use crate::{WmsAggregateWarehouseModel, WmsWarehouseSnap};

pub fn aggregate_to_snap(model: &WmsAggregateWarehouseModel) -> WmsWarehouseSnap {
    WmsWarehouseSnap {
        id: model.id,
        name: model.name.clone(),
        code: model.code.clone(),
        is_enabled: model.is_enabled,
        location: model.location.clone(),
        location_detail: model.location_detail.clone(),
    }
}
