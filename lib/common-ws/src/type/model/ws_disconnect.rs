use actix::{ActorContext, Handler, Message};

use crate::r#type::model::ws_session::WsSession;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsDisconnect;

impl Handler<WsDisconnect> for WsSession {
    type Result = ();

    fn handle(&mut self, _: WsDisconnect, ctx: &mut Self::Context) {
        ctx.close(None);
        ctx.stop();
    }
}
