use sea_orm::prelude::DateTimeWithTimeZone;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    model::crm_model::{
        account_model::CrmAggregateAccountModel, merchant_model::CrmAggregateMerchantModel,
    },
    EntityStatus,
};

#[derive(Serialize, ToSchema, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommonBaseRecordResp {
    /// 唯一标识
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 实体状态
    #[schema(example = "ACTIVE")]
    pub ent_st: EntityStatus,

    /// 创建人
    pub created_by: CrmAggregateAccountModel,

    /// 创建时间
    #[schema(value_type = String, format = DateTime, example = "2025-09-25T08:00:00Z")]
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Serialize, ToSchema, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommonRecordResp {
    /// 租户
    pub merchant: CrmAggregateMerchantModel,

    #[serde(flatten)]
    pub base: CommonBaseRecordResp,
}
