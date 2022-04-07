use std::sync::Arc;

use actix::{prelude::*, Actor};
use common::model::event::v1::auth::SendOtpMessage;
use lettre::{Message as LettreMessage, SmtpTransport, Transport};

/// Email Sender
pub struct EmailSender {
    pub smtp_mailer: Arc<SmtpTransport>,
}
impl Actor for EmailSender {
    type Context = Context<Self>;
}

impl Handler<SendOtpMessage> for EmailSender {
    type Result = Result<(), std::io::Error>;
    fn handle(&mut self, msg: SendOtpMessage, ctx: &mut Self::Context) -> Self::Result {
        let email = LettreMessage::builder()
            .from(msg.from.parse().unwrap())
            .to(msg.to.parse().unwrap())
            .subject(msg.sub)
            .body(msg.body)
            .unwrap();

        // #TODO return response and error
        self.smtp_mailer.send(&email);

        Ok(())
    }
}
