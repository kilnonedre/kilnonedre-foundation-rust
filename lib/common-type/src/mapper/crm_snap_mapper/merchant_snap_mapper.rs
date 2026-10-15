use crate::{
    CrmAggregateMerchantModel, CrmCompositeMerchantModel, CrmMerchantModel, CrmMerchantSnap,
};

pub fn model_to_snap(model: &CrmMerchantModel) -> CrmMerchantSnap {
    CrmMerchantSnap {
        id: model.id,
        name: model.name.clone(),
        code: model.code.clone(),
        license_no: model.license_no.clone(),
        logo_id: model.logo_id,
    }
}

pub fn composite_to_snap(model: &CrmCompositeMerchantModel) -> CrmMerchantSnap {
    CrmMerchantSnap {
        id: model.id,
        name: model.name.clone(),
        code: model.code.clone(),
        license_no: model.license_no.clone(),
        logo_id: model.logo_id,
    }
}

pub fn aggregate_to_snap(model: &CrmAggregateMerchantModel) -> CrmMerchantSnap {
    CrmMerchantSnap {
        id: model.id,
        name: model.name.clone(),
        code: model.code.clone(),
        license_no: model.license_no.clone(),
        logo_id: model.logo_id,
    }
}
