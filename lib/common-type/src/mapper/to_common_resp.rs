use std::collections::HashMap;

use sea_orm::entity::prelude::DateTimeWithTimeZone;
use uuid::Uuid;

use crate::{
    model::common_resp::CommonBaseResp,
    util::lookup::{lookup_optional, lookup_required},
    CommonResp, CrmAggregateAccountModel, CrmAggregateMerchantModel, EntityStatus,
};

pub fn to_common_resp(
    merchant_map: &HashMap<Uuid, CrmAggregateMerchantModel>,
    account_map: &HashMap<Uuid, CrmAggregateAccountModel>,
    id: &Uuid,
    ent_st: &EntityStatus,
    merchant_id: &Uuid,
    created_by: &Uuid,
    created_at: &DateTimeWithTimeZone,
    updated_by: &Option<Uuid>,
    updated_at: &Option<DateTimeWithTimeZone>,
    updated_reason: &Option<String>,
) -> CommonResp {
    CommonResp {
        merchant: lookup_required(merchant_id, merchant_map),
        base: to_common_base_resp(
            account_map,
            id,
            ent_st,
            created_by,
            created_at,
            updated_by,
            updated_at,
            updated_reason,
        ),
    }
}

pub fn to_common_base_resp(
    account_map: &HashMap<Uuid, CrmAggregateAccountModel>,
    id: &Uuid,
    ent_st: &EntityStatus,
    created_by: &Uuid,
    created_at: &DateTimeWithTimeZone,
    updated_by: &Option<Uuid>,
    updated_at: &Option<DateTimeWithTimeZone>,
    updated_reason: &Option<String>,
) -> CommonBaseResp {
    CommonBaseResp {
        id: *id,
        ent_st: *ent_st,
        created_by: lookup_required(created_by, account_map),
        created_at: *created_at,
        updated_by: lookup_optional(updated_by, account_map),
        updated_at: *updated_at,
        updated_reason: updated_reason.clone(),
    }
}
