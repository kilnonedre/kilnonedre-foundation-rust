use crate::{
    snapshot::logistics_snap::area_snap::LogisticsAreaSnap, LogisticsAggregateAreaModel,
    LogisticsAreaModel,
};

pub fn aggregate_to_snap(model: &LogisticsAggregateAreaModel) -> LogisticsAreaSnap {
    LogisticsAreaSnap {
        id: model.id,
        name: model.name.clone(),
        code: model.code.clone(),
    }
}

pub fn model_to_snap(model: &LogisticsAreaModel) -> LogisticsAreaSnap {
    LogisticsAreaSnap {
        id: model.id,
        name: model.name.clone(),
        code: model.code.clone(),
    }
}
