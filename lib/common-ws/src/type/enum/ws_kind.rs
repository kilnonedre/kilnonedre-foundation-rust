use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WsKind {
    /// 系统事件
    System,

    /// 实体数据事件
    Entity,
}
