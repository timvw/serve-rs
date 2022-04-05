use tonic::{transport::Server, Request, Response, Status};
use futures_core::Stream;
use std::pin::Pin;
use std::thread;
use std::time::Duration;
use log::info;
use chrono::Utc;

use serve::broker::{PublishRequest, PublishResponse, SubscribeRequest};
use serve::broker::broker_server::{BrokerServer, Broker};

#[derive(Debug, Default)]
pub struct MyBroker {}

#[tonic::async_trait]
impl Broker for MyBroker {

    async fn publish(&self, request: Request<PublishRequest>) -> Result<Response<PublishResponse>, Status> {
        info!("Got a request: {:?}", request);

        let reply = PublishResponse {
            message: format!("Hello {}!", request.into_inner().message).into(),
        };

        Ok(Response::new(reply))
    }

    type SubscribeStream = Pin<Box<dyn Stream<Item = Result<serve::broker::Message, Status>> + Send + Sync + 'static>>;

    async fn subscribe(&self, _request: Request<SubscribeRequest>) -> Result<Response<Self::SubscribeStream>, Status> {

        let output = async_stream::try_stream! {
            loop {
                let book_message = serve::broker::Message {
                    message: format!("The time is {:?}", Utc::now()),
                };

                info!("yielding: {:?}", book_message);
                yield book_message
            }
        };

        Ok(Response::new(Box::pin(output) as Self::SubscribeStream))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"));

    let addr = "[::1]:50051".parse()?;
    let broker = MyBroker::default();

    Server::builder()
        .add_service(BrokerServer::new(broker))
        .serve(addr)
        .await?;

    Ok(())
}