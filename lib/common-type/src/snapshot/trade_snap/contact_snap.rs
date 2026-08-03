use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::snapshot::{
    crm_snap::account_snap::CrmAccountSnap, trade_snap::organization_snap::TradeOrganizationSnap,
};

/// 联系人快照
#[derive(Serialize, ToSchema, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeContactSnap {
    /// ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 信息
    pub consumer_profile: CrmAccountSnap,

    /// 组织
    pub organization: TradeOrganizationSnap,

    /// 是否为默认联系人
    #[schema(example = false)]
    pub is_default: bool,

    /// 职位
    #[schema(example = "xxxxxxxx")]
    pub position: Option<String>,
}
