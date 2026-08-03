use std::collections::HashMap;

use uuid::Uuid;

use crate::{
    account_mapper, lookup_required, mapper::logistics_mapper::car_mapper,
    snapshot::logistics_snap::driver_snap::LogisticsDriverSnap, CrmAggregateAccountModel,
    LogisticsAggregateDriverModel, LogisticsDriverModel,
};

pub fn aggregate_to_snap(model: &LogisticsAggregateDriverModel) -> LogisticsDriverSnap {
    LogisticsDriverSnap {
        id: model.id,
        seller_profile: account_mapper::model_to_snap(&model.seller_profile),
        car: model
            .car
            .as_ref()
            .map(|car| car_mapper::model_to_snap(&car)),
        driving_license_no: model.driving_license_no.clone(),
        certificate_ids: model.certificate_ids.clone(),
    }
}

pub fn model_to_snap(
    model: &LogisticsDriverModel,
    account_map: &HashMap<Uuid, CrmAggregateAccountModel>,
) -> LogisticsDriverSnap {
    LogisticsDriverSnap {
        id: model.id,
        seller_profile: account_mapper::aggregate_to_snap(&lookup_required(
            &model.seller_profile_id,
            account_map,
        )),
        car: model
            .car
            .as_ref()
            .map(|car| car_mapper::model_to_snap(&car)),
        driving_license_no: model.driving_license_no.clone(),
        certificate_ids: model.certificate_ids.clone(),
    }
}
