use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// 账户快照
#[derive(Serialize, ToSchema, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrmAccountSnap {
    /// 账户 ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

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
