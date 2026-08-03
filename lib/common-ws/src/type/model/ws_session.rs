use common_type::OperatorType;
use std::{sync::Arc, time::Instant};
use uuid::Uuid;

use crate::{hub::WsHub, session::OnTextHandler};

pub struct WsSession {
    /// 会话 ID
    pub session_id: Uuid,

    /// 当前用户 ID
    pub user_id: Uuid,

    /// 当前操作人类型
    pub operator_type: OperatorType,

    /// 当前商户 ID
    pub merchant_id: Uuid,

    /// 最后心跳时间
    pub hb: Instant,

    /// WebSocket 连接管理器
    pub manager: Arc<WsHub>,

    /// 文本消息回调
    pub on_text: OnTextHandler,
}
