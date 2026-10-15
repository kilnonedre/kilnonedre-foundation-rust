use std::collections::HashMap;

use uuid::Uuid;

use crate::{lookup_required, merchant_snap_mapper, CommonSnap, CrmAggregateMerchantModel};

pub fn to_common_snap(
    id: &Uuid,
    merchant_id: &Uuid,
    merchant_map: &HashMap<Uuid, CrmAggregateMerchantModel>,
) -> CommonSnap {
    let merchant = lookup_required(merchant_id, merchant_map);
    CommonSnap {
        id: *id,
        merchant_snap: merchant_snap_mapper::aggregate_to_snap(&merchant),
    }
}
