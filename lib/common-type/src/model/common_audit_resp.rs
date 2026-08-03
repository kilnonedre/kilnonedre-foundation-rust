use sea_orm::prelude::DateTimeWithTimeZone;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    model::crm_model::{
        account_model::CrmAggregateAccountModel, merchant_model::CrmAggregateMerchantModel,
    },
    AuditStatus, EntityStatus,
};

pub trait HasAuditMeta {
    fn meta(&self) -> &CommonAuditResp;
}

#[derive(Serialize, ToSchema, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommonAuditBaseResp {
    /// 唯一标识
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 审计状态
    #[schema(example = "CREATE")]
    pub aud_st: AuditStatus,

    /// 原实体 ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub ent_id: Uuid,

    /// 实体状态
    #[schema(example = "ACTIVE")]
    pub ent_st: EntityStatus,

    /// 创建人
    pub created_by: CrmAggregateAccountModel,

    /// 创建时间
    #[schema(value_type = String, format = DateTime, example = "2025-09-25T08:00:00Z")]
    pub created_at: DateTimeWithTimeZone,

    /// 更新人
    pub updated_by: Option<CrmAggregateAccountModel>,

    /// 更新时间
    #[schema(value_type = String, format = DateTime, example = "2025-09-25T08:00:00Z")]
    pub updated_at: Option<DateTimeWithTimeZone>,

    /// 更新原因
    #[schema(example = "分类名称调整")]
    pub updated_reason: Option<String>,
}

#[derive(Serialize, ToSchema, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommonAuditResp {
    /// 租户
    pub merchant: CrmAggregateMerchantModel,

    #[serde(flatten)]
    pub base: CommonAuditBaseResp,
}
