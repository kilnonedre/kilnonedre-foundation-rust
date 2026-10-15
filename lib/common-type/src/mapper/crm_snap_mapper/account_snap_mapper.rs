use std::collections::HashMap;

use uuid::Uuid;

use crate::{
    snapshot::crm_snap::account_snap::CrmAccountSnap, to_common_snap, CrmAccountModel,
    CrmAggregateAccountModel, CrmCompositeAccountModel, CrmMerchantSnap,
};

pub fn model_to_snap(
    model: &CrmAccountModel,
    merchant_snap_map: &HashMap<Uuid, CrmMerchantSnap>,
) -> CrmAccountSnap {
    CrmAccountSnap {
        profile_id: model.profile_id,
        username: model.username.clone(),
        handle: model.handle.clone(),
        email: model.email.clone(),
        phone: model.phone.clone(),
        avatar_id: model.avatar_id,
        meta: to_common_snap::model_to_common_snap(
            &model.id,
            &model.merchant_id,
            merchant_snap_map,
        ),
    }
}

pub fn composite_to_snap(model: &CrmCompositeAccountModel) -> CrmAccountSnap {
    CrmAccountSnap {
        profile_id: model.profile_id,
        username: model.username.clone(),
        handle: model.handle.clone(),
        email: model.email.clone(),
        phone: model.phone.clone(),
        avatar_id: model.avatar_id,
        meta: to_common_snap::composite_to_common_snap(&model.id, &model.merchant),
    }
}

pub fn aggregate_to_snap(model: &CrmAggregateAccountModel) -> CrmAccountSnap {
    CrmAccountSnap {
        profile_id: model.profile_id,
        username: model.username.clone(),
        handle: model.handle.clone(),
        email: model.email.clone(),
        phone: model.phone.clone(),
        avatar_id: model.avatar_id,
        meta: to_common_snap::aggregate_to_common_snap(&model.id, &model.merchant),
    }
}
