use actix::{Handler, Message};
use serde::{Deserialize, Serialize};

use crate::r#type::{model::ws_session::WsSession, r#enum::ws_kind::WsKind};

#[derive(Debug, Clone, Serialize, Deserialize, Message)]
#[rtype(result = "()")]
#[serde(rename_all = "camelCase")]
pub struct WsMessage {
    /// 类型
    pub kind: WsKind,

    /// 事件
    pub event: String,

    /// 来源
    pub source: Option<String>,
}

impl Handler<WsMessage> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        match serde_json::to_string(&msg) {
            Ok(text) => ctx.text(text),
            Err(e) => {
                log::error!("WebSocket 消息序列化失败: {e}");
            }
        }
    }
}
