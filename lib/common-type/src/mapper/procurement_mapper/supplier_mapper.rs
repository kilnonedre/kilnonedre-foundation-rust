use std::collections::HashMap;

use uuid::Uuid;

use crate::{
    account_mapper, lookup_required,
    snapshot::procurement_snap::supplier_snap::ProcurementSupplierSnap, CrmAggregateAccountModel,
    GeoLocationResp, ProcurementAggregateSupplierModel, ProcurementSupplierModel,
};

pub fn aggregate_to_snap(
    model: &ProcurementAggregateSupplierModel,
    location_map: &HashMap<Uuid, GeoLocationResp>,
) -> ProcurementSupplierSnap {
    let location = lookup_required(&model.location_id, location_map);
    ProcurementSupplierSnap {
        id: model.id,
        name: model.name.clone(),
        code: model.code.clone(),
        seller_profile: account_mapper::model_to_snap(&model.seller_profile),
        location: location.clone(),
        location_detail: model.location_detail.clone(),
        certificate_ids: model.certificate_ids.clone(),
    }
}

pub fn model_to_snap(
    model: &ProcurementSupplierModel,
    account_map: &HashMap<Uuid, CrmAggregateAccountModel>,
    location_map: &HashMap<Uuid, GeoLocationResp>,
) -> ProcurementSupplierSnap {
    let account = lookup_required(&model.seller_profile_id, account_map);
    let location = lookup_required(&model.location_id, location_map);
    ProcurementSupplierSnap {
        id: model.id,
        name: model.name.clone(),
        code: model.code.clone(),
        seller_profile: account_mapper::aggregate_to_snap(&account),
        location: location.clone(),
        location_detail: model.location_detail.clone(),
        certificate_ids: model.certificate_ids.clone(),
    }
}
