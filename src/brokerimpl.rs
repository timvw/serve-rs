use tonic::{Request, Response, Status};
use futures_core::Stream;
use std::pin::Pin;
use log::info;
use chrono::Utc;
use tokio::sync::{mpsc, oneshot, watch};
use tokio::sync::mpsc::{Receiver, Sender};

use crate::broker::{PublishRequest, PublishResponse, SubscribeRequest};
use crate::broker::broker_server::Broker;
use crate::topic::Topic;

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

        let res = self.topic.publish(request.into_inner().message).await;

        Ok(Response::new(PublishResponse {
            message: format!("offset: {:?}", res.offset),
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