use std::collections::HashMap;

use uuid::Uuid;

use crate::{
    lookup_required, merchant_snap_mapper, CommonSnap, CrmAggregateMerchantModel,
    CrmCompositeMerchantModel, CrmMerchantSnap,
};

pub fn model_to_common_snap(
    id: &Uuid,
    merchant_id: &Uuid,
    merchant_snap_map: &HashMap<Uuid, CrmMerchantSnap>,
) -> CommonSnap {
    CommonSnap {
        id: *id,
        merchant_snap: lookup_required(merchant_id, merchant_snap_map),
    }
}

pub fn composite_to_common_snap(id: &Uuid, merchant: &CrmCompositeMerchantModel) -> CommonSnap {
    CommonSnap {
        id: *id,
        merchant_snap: merchant_snap_mapper::composite_to_snap(merchant),
    }
}

pub fn aggregate_to_common_snap(id: &Uuid, merchant: &CrmAggregateMerchantModel) -> CommonSnap {
    CommonSnap {
        id: *id,
        merchant_snap: merchant_snap_mapper::aggregate_to_snap(merchant),
    }
}
