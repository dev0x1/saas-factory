use crate::context::AppContext;
use actix::{Actor, Context, Handler, Recipient};
use common::model::event::{
    v1::{auth::SendOtpMessage, Event},
    EventMessage,
};
use std::sync::Arc;
use tracing::info;

pub struct EventStreamHandler {
    pub context: Arc<AppContext>,
    pub email_sender: Recipient<SendOtpMessage>,
}

impl EventStreamHandler {
    // Define handler for `SendOtp` message
    fn process_send_otp(&self, event_message: EventMessage<SendOtpMessage>) {
        info!("Processing SendOtp command...: {:?}", event_message);
        self.email_sender.do_send(event_message.payload);
    }
}

// Provide Actor implementation for our actor
impl Actor for EventStreamHandler {
    type Context = Context<Self>;
}

// Define handler for `Event` message
impl Handler<Event> for EventStreamHandler {
    type Result = Result<(), std::io::Error>;

    fn handle(&mut self, event: Event, ctx: &mut Context<Self>) -> Self::Result {
        println!("Handling event: {:?}", event);
        match event {
            Event::AuthSendOtp(event_message) => self.process_send_otp(event_message),
            _ => {
                println!("Unprocessed event: {:?}", event);
            }
        }

        Ok(())
    }
}
