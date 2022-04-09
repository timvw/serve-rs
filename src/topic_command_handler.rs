use log::info;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
pub enum TopicCommand {
    Publish(PublishRequest),
}

#[derive(Debug)]
pub struct PublishRequest {
    pub message: String,
    pub responder: oneshot::Sender<PublishResponse>,
}

#[derive(Debug)]
pub struct PublishResponse {
    pub offset: u64,
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
            TopicCommand::Publish(publish_request) => self.handle_publish_request(publish_request)
        }
    }

    pub fn handle_publish_request(&mut self, publish_request: PublishRequest) {
        info!("changing current from {} into: {}", self.message, publish_request.message);
        let response = PublishResponse { offset: self.offset };
        self.message = publish_request.message;
        self.offset += 1;
        let _ = publish_request.responder.send(response);
    }
}
