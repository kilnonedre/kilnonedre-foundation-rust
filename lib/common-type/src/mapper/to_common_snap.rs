use std::collections::HashMap;

use uuid::Uuid;

use crate::{lookup_required, CommonSnap, CrmMerchantSnap};

pub fn to_common_snap(
    id: &Uuid,
    merchant_id: &Uuid,
    merchant_snap_map: &HashMap<Uuid, CrmMerchantSnap>,
) -> CommonSnap {
    CommonSnap {
        id: *id,
        merchant_snap: lookup_required(merchant_id, merchant_snap_map),
    }
}
