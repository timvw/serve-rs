use log::trace;
use tokio::sync::oneshot;
use crate::topic::{PublishRequest, PublishResponse};

#[derive(Debug)]
pub enum TopicCommand {
    Publish(PublishRequest, oneshot::Sender<PublishResponse>,),
}

#[derive(Debug)]
pub struct TopicCommandHandler {
    offset: u64,
    message: String,
}

impl TopicCommandHandler {
    pub fn new() -> TopicCommandHandler {
        TopicCommandHandler {
            offset: 0,
            message: "".to_string(),
        }
    }

    pub fn handle(&mut self, cmd: TopicCommand) {
        match cmd {
            TopicCommand::Publish(publish_request, response_sender) => self.handle_publish_request(publish_request, response_sender)
        }
    }

    pub fn handle_publish_request(&mut self, publish_request: PublishRequest, response_sender: oneshot::Sender<PublishResponse>) {
        trace!("changing current from {} into: {}", self.message, publish_request.message);
        let response = PublishResponse { offset: self.offset };
        self.message = publish_request.message;
        self.offset += 1;
        let _ = response_sender.send(response);
    }
}
