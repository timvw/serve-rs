use log::info;
use tokio::sync::mpsc::Sender;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
enum TopicCommand {
    Publish {
        message: String,
        responder: oneshot::Sender<PublishResponse>,
    }
}

#[derive(Debug)]
pub struct PublishResponse {
    pub offset: u64,
}

#[derive(Debug)]
pub struct Topic {
    command_sender: Sender<TopicCommand>,
}

impl Topic {
    pub fn new() -> Topic {
        let (command_sender, mut command_receiver) = mpsc::channel::<TopicCommand>(32);

        let manager = tokio::spawn(async move {
            let mut current: String = "".to_string();
            let mut offset: u64 = 0;

            while let Some(cmd) = command_receiver.recv().await {
                match cmd {
                    TopicCommand::Publish { message, responder } => {
                        info!("changing current from {} into: {}", current, message);
                        current = message;
                        let response = PublishResponse { offset };
                        responder.send(response);
                        offset += 1;
                    }
                }
            }
        });

        Topic {
            command_sender: command_sender,
        }
    }

    pub async fn publish(&self, message: String) -> PublishResponse {

        let command_sender = self.command_sender.clone();
        let (command_response_sender, command_response_receiver) = oneshot::channel();

        let s = tokio::spawn(async move {
            let publish_command = TopicCommand::Publish {
                message: message,
                responder: command_response_sender,
            };
            command_sender.send(publish_command).await.unwrap();
            command_response_receiver.await
        });

        s.await.unwrap().unwrap()
    }
}
