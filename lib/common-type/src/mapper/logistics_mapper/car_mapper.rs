use crate::{snapshot::logistics_snap::car_snap::LogisticsCarSnap, LogisticsCarModel};

// pub fn aggregate_to_snap(model: &LogisticsAggregateCarModel) -> LogisticsCarSnap {
//     LogisticsCarSnap {
//         id: model.id,
//         plate_no: model.plate_no.clone(),
//         capacity_weight: model.capacity_weight,
//         capacity_volume: model.capacity_volume,
//     }
// }

pub fn model_to_snap(model: &LogisticsCarModel) -> LogisticsCarSnap {
    LogisticsCarSnap {
        id: model.id,
        plate_no: model.plate_no.clone(),
        capacity_weight: model.capacity_weight,
        capacity_volume: model.capacity_volume,
    }
}
