use crate::{
    CrmAggregateMerchantModel, CrmCompositeMerchantModel, CrmMerchantModel, CrmMerchantSnap,
};

pub fn model_to_snap(model: &CrmMerchantModel) -> CrmMerchantSnap {
    CrmMerchantSnap {
        id: model.id,
        name: model.name.clone(),
        code: model.code.clone(),
    }
}

pub fn composite_to_snap(model: &CrmCompositeMerchantModel) -> CrmMerchantSnap {
    CrmMerchantSnap {
        id: model.id,
        name: model.name.clone(),
        code: model.code.clone(),
    }
}

pub fn aggregate_to_snap(model: &CrmAggregateMerchantModel) -> CrmMerchantSnap {
    CrmMerchantSnap {
        id: model.id,
        name: model.name.clone(),
        code: model.code.clone(),
    }
}
