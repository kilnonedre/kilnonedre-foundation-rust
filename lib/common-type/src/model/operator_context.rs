use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::r#enum::operator_type::OperatorType;

#[derive(Serialize, ToSchema, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OperatorContext {
    /// 操作人类型（ADMIN / SELLER / CONSUMER）
    #[schema(example = "ADMIN")]
    pub operator_type: OperatorType,

    /// 操作人用户ID（UUID）
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub user_id: Uuid,

    /// 商户ID（UUID）
    #[schema(example = "550e8400-e29b-41d4-a716-446655440001")]
    pub merchant_id: Uuid,
}
