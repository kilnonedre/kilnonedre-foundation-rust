use std::collections::HashMap;

use uuid::Uuid;

use crate::{
    account_mapper, lookup_required,
    snapshot::procurement_snap::purchaser_snap::ProcurementPurchaserSnap, CrmAggregateAccountModel,
    ProcurementAggregatePurchaserModel, ProcurementPurchaserModel,
};

pub fn aggregate_to_snap(model: &ProcurementAggregatePurchaserModel) -> ProcurementPurchaserSnap {
    ProcurementPurchaserSnap {
        id: model.id,
        seller_profile: account_mapper::model_to_snap(&model.seller_profile),
    }
}

pub fn model_to_snap(
    model: &ProcurementPurchaserModel,
    account_map: &HashMap<Uuid, CrmAggregateAccountModel>,
) -> ProcurementPurchaserSnap {
    let account = lookup_required(&model.seller_profile_id, account_map);
    ProcurementPurchaserSnap {
        id: model.id,
        seller_profile: account_mapper::aggregate_to_snap(&account),
    }
}
