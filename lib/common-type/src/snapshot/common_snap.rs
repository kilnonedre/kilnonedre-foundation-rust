use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::CrmMerchantSnap;

#[derive(Serialize, ToSchema, Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonSnap {
    /// 唯一标识
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 租户
    pub merchant_snap: CrmMerchantSnap,
}
