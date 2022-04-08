use tokio::sync::mpsc::Sender;
use tokio::sync::{mpsc, oneshot};
use crate::topiccommandhandler::{PublishRequest, PublishResponse, TopicCommand, TopicCommandHandler};

#[derive(Debug)]
pub struct Topic {
    command_sender: Sender<TopicCommand>,
}

impl Topic {
    pub fn new() -> Topic {
        let (command_sender, command_receiver) = mpsc::channel::<TopicCommand>(32);

        let _ = tokio::spawn(async move {
            let mut topic_manager = TopicCommandHandler::new();
            topic_manager.run(command_receiver).await;
        });

        Topic {
            command_sender: command_sender,
        }
    }

    pub async fn publish(&self, message: String) -> PublishResponse {

        let command_sender = self.command_sender.clone();
        let (command_response_sender, command_response_receiver) = oneshot::channel();

        let s = tokio::spawn(async move {
            let publish_command = TopicCommand::Publish(PublishRequest {
                message: message,
                responder: command_response_sender,
            });
            command_sender.send(publish_command).await.unwrap();
            command_response_receiver.await
        });

        s.await.unwrap().unwrap()
    }
}
