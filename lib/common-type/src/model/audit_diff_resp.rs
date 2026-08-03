use sea_orm::prelude::DateTimeWithTimeZone;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    model::crm_model::account_model::CrmAggregateAccountModel, r#enum::audit_status::AuditStatus,
};

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuditDiffResp {
    /// 审计记录的唯一标识
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 审计操作类型
    #[schema(example = "UPDATE")]
    pub action: AuditStatus,

    /// 操作人
    pub operator: Option<CrmAggregateAccountModel>,

    /// 操作时间
    #[schema(value_type = String, format = DateTime, example = "2025-09-25T08:00:00Z")]
    pub operated_at: Option<DateTimeWithTimeZone>,

    /// 更新原因
    #[schema(example = "分类名称调整")]
    pub updated_reason: Option<String>,

    /// 字段变更明细
    pub changes: Vec<AuditFieldChangeResp>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuditFieldChangeResp {
    /// 字段名
    #[schema(example = "name")]
    pub field: String,

    /// 字段显示名称
    #[schema(example = "分类名称")]
    pub label: String,

    /// 修改前的显示值
    #[schema(example = "蔬菜")]
    pub old_text: Option<String>,

    /// 修改后的显示值
    #[schema(example = "新鲜蔬菜")]
    pub new_text: Option<String>,
}
