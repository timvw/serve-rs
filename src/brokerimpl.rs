use tonic::{Request, Response, Status};
use futures_core::Stream;
use std::pin::Pin;
use log::info;
use chrono::Utc;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::broker::{PublishRequest, PublishResponse, SubscribeRequest};
use crate::broker::broker_server::Broker;

#[derive(Debug)]
enum Command {
    Get {
    },
    Set {
        val: String,
    }
}

#[derive(Debug)]
pub struct Topic {
    tx: Sender<Command>,
}

impl Topic {
    pub fn new() -> Topic {
        let (tx, mut rx) = mpsc::channel::<Command>(32);

        let manager = tokio::spawn(async move {
            let mut current: String = "".to_string();

            while let Some(cmd) = rx.recv().await {
                use Command::*;

                match cmd {
                    Get {  } => {
                        info!("need to return current value: {}", current);
                    }
                    Set { val } => {
                        info!("changing current from {} into: {}", current, val);
                        current = val;
                    }
                }
            }
        });

        Topic {
            tx: tx,
        }
    }
}

#[derive(Debug)]
pub struct BrokerImpl {
    topic: Topic,
}

impl BrokerImpl {
    pub fn new() -> BrokerImpl {
        BrokerImpl {
            topic: Topic::new(),
        }
    }
}

#[tonic::async_trait]
impl Broker for BrokerImpl {

    async fn publish(&self, request: Request<PublishRequest>) -> Result<Response<PublishResponse>, Status> {
        info!("Got a request: {:?}", request);

        let tx = self.topic.tx.clone();
        let s = tokio::spawn(async move {
            let cmd = Command::Set {
                val: request.into_inner().message,
            };

            tx.send(cmd).await.unwrap();
        });

        s.await.unwrap();
        Ok(Response::new(PublishResponse {
            message: "ok".to_string(),
        }))
    }

    type SubscribeStream = Pin<Box<dyn Stream<Item = Result<crate::broker::Message, Status>> + Send + Sync + 'static>>;

    async fn subscribe(&self, _request: Request<SubscribeRequest>) -> Result<Response<Self::SubscribeStream>, Status> {

        let output = async_stream::try_stream! {
            loop {
                let book_message = crate::broker::Message {
                    message: format!("The time is {:?}", Utc::now()),
                };

                info!("yielding: {:?}", book_message);
                yield book_message
            }
        };

        Ok(Response::new(Box::pin(output) as Self::SubscribeStream))
    }
}