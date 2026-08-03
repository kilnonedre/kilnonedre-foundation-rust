use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DeriveActiveEnum, EnumIter, ToSchema,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "point_record_biz_type"
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PointRecordBizType {
    /// 邀请新用户奖励
    #[sea_orm(string_value = "INVITE_REGISTER")]
    InviteRegister,

    /// 消费积分
    #[sea_orm(string_value = "ORDER_CONSUME")]
    OrderConsume,
}
