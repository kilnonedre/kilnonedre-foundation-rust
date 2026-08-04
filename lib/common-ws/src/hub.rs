use actix::Recipient;
use dashmap::DashMap;
use kilnonedre_common_type::OperatorType;
use uuid::Uuid;

use crate::r#type::{
    model::{ws_disconnect::WsDisconnect, ws_message::WsMessage},
    r#enum::ws_kind::WsKind,
};

pub struct WsRecipients {
    pub disconnect: Recipient<WsDisconnect>,
    pub message: Recipient<WsMessage>,
}

pub struct WsSessionItem {
    pub user_id: Uuid,

    pub operator_type: OperatorType,

    pub merchant_id: Uuid,

    pub recipients: WsRecipients,
}

pub struct WsHub {
    sessions: DashMap<Uuid, WsSessionItem>,
}

impl WsHub {
    pub fn new() -> Self {
        Self {
            sessions: DashMap::new(),
        }
    }

    pub fn add_session(
        &self,
        user_id: Uuid,
        operator_type: OperatorType,
        merchant_id: Uuid,
        session_id: Uuid,
        disconnect: Recipient<WsDisconnect>,
        message: Recipient<WsMessage>,
    ) {
        self.sessions.insert(
            session_id,
            WsSessionItem {
                user_id,
                operator_type,
                merchant_id,
                recipients: WsRecipients {
                    disconnect,
                    message,
                },
            },
        );
    }

    pub fn remove_session(&self, _user_id: &Uuid, session_id: &Uuid) {
        self.sessions.remove(session_id);
    }

    pub fn send(
        &self,
        user_id: Option<Uuid>,
        operator_type: Option<OperatorType>,
        merchant_id: Option<Uuid>,
        message: WsMessage,
    ) {
        for item in self.sessions.iter() {
            let session = item.value();

            if let Some(v) = user_id {
                if session.user_id != v {
                    continue;
                }
            }

            if let Some(v) = operator_type {
                if session.operator_type != v {
                    continue;
                }
            }

            if let Some(v) = merchant_id {
                if session.merchant_id != v {
                    continue;
                }
            }

            let _ = session.recipients.message.do_send(message.clone());
        }
    }

    pub fn disconnect(
        &self,
        user_id: Option<Uuid>,
        operator_type: Option<OperatorType>,
        merchant_id: Option<Uuid>,
        reason: impl Into<String>,
    ) {
        let reason = reason.into();

        let session_ids: Vec<Uuid> = self
            .sessions
            .iter()
            .filter_map(|item| {
                let session = item.value();

                if let Some(v) = user_id {
                    if session.user_id != v {
                        return None;
                    }
                }

                if let Some(v) = operator_type {
                    if session.operator_type != v {
                        return None;
                    }
                }

                if let Some(v) = merchant_id {
                    if session.merchant_id != v {
                        return None;
                    }
                }

                Some(*item.key())
            })
            .collect();

        for session_id in session_ids {
            if let Some((_, session)) = self.sessions.remove(&session_id) {
                let _ = session.recipients.message.do_send(WsMessage {
                    kind: WsKind::System,
                    event: "FORCE_OFFLINE".to_string(),
                    source: Some(reason.clone()),
                });

                let _ = session.recipients.disconnect.do_send(WsDisconnect);
            }
        }
    }
}
