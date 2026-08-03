use crate::{
    snapshot::crm_snap::account_snap::CrmAccountSnap, CrmAccountModel, CrmAggregateAccountModel,
};

pub fn aggregate_to_snap(model: &CrmAggregateAccountModel) -> CrmAccountSnap {
    CrmAccountSnap {
        id: model.id,
        profile_id: model.profile_id,
        username: model.username.clone(),
        handle: model.handle.clone(),
        email: model.email.clone(),
        phone: model.phone.clone(),
        avatar_id: model.avatar_id,
    }
}

pub fn model_to_snap(model: &CrmAccountModel) -> CrmAccountSnap {
    CrmAccountSnap {
        id: model.id,
        profile_id: model.profile_id,
        username: model.username.clone(),
        handle: model.handle.clone(),
        email: model.email.clone(),
        phone: model.phone.clone(),
        avatar_id: model.avatar_id,
    }
}
