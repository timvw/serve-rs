use tokio::sync::mpsc::Sender;
use tokio::sync::{mpsc, oneshot};
use crate::topic_command_handler::{TopicCommand, TopicCommandHandler};

#[derive(Debug)]
pub struct Topic {
    command_sender: Sender<TopicCommand>,
}

impl Topic {
    pub fn new() -> Topic {
        let (command_sender, mut command_receiver) = mpsc::channel::<TopicCommand>(32);

        let _ = tokio::spawn(async move {
            let mut topic_command_handler = TopicCommandHandler::new();
            while let Some(cmd) = command_receiver.recv().await {
                topic_command_handler.handle(cmd);
            }
        });

        Topic {
            command_sender: command_sender,
        }
    }

    pub async fn publish(&self, publish_request: PublishRequest) -> PublishResponse {

        let command_sender = self.command_sender.clone();
        let (command_response_sender, command_response_receiver) = oneshot::channel();

        let s = tokio::spawn(async move {
            let publish_command = TopicCommand::Publish(publish_request, command_response_sender);
            command_sender.send(publish_command).await.unwrap();
            command_response_receiver.await
        });

        s.await.unwrap().unwrap()
    }
}

#[derive(Debug)]
pub struct PublishRequest {
    pub message: String,
}

#[derive(Debug)]
pub struct PublishResponse {
    pub offset: u64,
}
