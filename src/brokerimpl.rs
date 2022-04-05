use tonic::{Request, Response, Status};
use futures_core::Stream;
use std::pin::Pin;
use log::info;
use chrono::Utc;

use crate::broker::{PublishRequest, PublishResponse, SubscribeRequest};
use crate::broker::broker_server::Broker;

#[derive(Debug, Default)]
pub struct BrokerImpl {}

#[tonic::async_trait]
impl Broker for BrokerImpl {

    async fn publish(&self, request: Request<PublishRequest>) -> Result<Response<PublishResponse>, Status> {
        info!("Got a request: {:?}", request);

        let reply = PublishResponse {
            message: format!("Hello {}!", request.into_inner().message).into(),
        };

        Ok(Response::new(reply))
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