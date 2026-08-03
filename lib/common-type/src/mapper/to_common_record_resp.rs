use std::collections::HashMap;

use sea_orm::entity::prelude::DateTimeWithTimeZone;
use uuid::Uuid;

use crate::{
    util::lookup::lookup_required, CommonBaseRecordResp, CommonRecordResp,
    CrmAggregateAccountModel, CrmAggregateMerchantModel, EntityStatus,
};

pub fn to_common_record_resp(
    merchant_map: &HashMap<Uuid, CrmAggregateMerchantModel>,
    account_map: &HashMap<Uuid, CrmAggregateAccountModel>,
    id: &Uuid,
    ent_st: &EntityStatus,
    merchant_id: &Uuid,
    created_by: &Uuid,
    created_at: &DateTimeWithTimeZone,
) -> CommonRecordResp {
    CommonRecordResp {
        merchant: lookup_required(merchant_id, merchant_map),
        base: to_common_base_record_resp(account_map, id, ent_st, created_by, created_at),
    }
}

pub fn to_common_base_record_resp(
    account_map: &HashMap<Uuid, CrmAggregateAccountModel>,
    id: &Uuid,
    ent_st: &EntityStatus,
    created_by: &Uuid,
    created_at: &DateTimeWithTimeZone,
) -> CommonBaseRecordResp {
    CommonBaseRecordResp {
        id: *id,
        ent_st: *ent_st,
        created_by: lookup_required(created_by, account_map),
        created_at: *created_at,
    }
}
