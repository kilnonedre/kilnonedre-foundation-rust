use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::CommonSnap;

/// 账户快照
#[derive(Serialize, ToSchema, Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CrmAccountSnap {
    /// 快照
    #[serde(flatten)]
    pub meta: CommonSnap,

    /// 身份 ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub profile_id: Uuid,

    /// 用户名
    #[schema(example = "username")]
    pub username: String,

    /// 用户唯一标识符
    #[schema(example = "example")]
    pub handle: String,

    /// 邮箱
    #[schema(example = "xxxx@xx.xxx")]
    pub email: Option<String>,

    /// 电话号码
    #[schema(example = "example")]
    pub phone: Option<String>,

    /// 头像 ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub avatar_id: Option<Uuid>,
}
