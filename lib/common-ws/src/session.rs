use std::sync::Arc;
use std::time::Instant;

use actix::{Actor, ActorContext, AsyncContext, StreamHandler};
use actix_web_actors::ws;
use kilnonedre_common_type::OperatorType;
use uuid::Uuid;

use crate::{
    hub::WsHub,
    r#type::{
        model::{ws_disconnect::WsDisconnect, ws_message::WsMessage, ws_session::WsSession},
        r#enum::ws_kind::WsKind,
    },
};

pub type OnTextHandler = Arc<dyn Fn(Uuid, String) + Send + Sync>;

impl WsSession {
    pub fn new(
        user_id: Uuid,
        operator_type: OperatorType,
        merchant_id: Uuid,
        manager: Arc<WsHub>,
        on_text: OnTextHandler,
    ) -> Self {
        Self {
            session_id: Uuid::new_v4(),
            user_id,
            operator_type,
            merchant_id,
            hb: Instant::now(),
            manager,
            on_text,
        }
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.manager.add_session(
            self.user_id,
            self.operator_type,
            self.merchant_id,
            self.session_id,
            ctx.address().recipient::<WsDisconnect>(),
            ctx.address().recipient::<WsMessage>(),
        );

        ctx.text(
            serde_json::to_string(&WsMessage {
                kind: WsKind::System,
                event: "CONNECTED".to_string(),
                source: None,
            })
            .unwrap_or_default(),
        );
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        self.manager.remove_session(&self.user_id, &self.session_id);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();

                ctx.pong(&msg);
            }

            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }

            Ok(ws::Message::Text(text)) => {
                self.hb = Instant::now();

                (self.on_text)(self.user_id, text.to_string());
            }

            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);

                ctx.stop();
            }

            Err(_) => {
                ctx.stop();
            }

            _ => {}
        }
    }
}
