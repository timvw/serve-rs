use tonic::{transport::Server, Request, Response, Status};
use futures_core::Stream;
use std::pin::Pin;
use serve::bitvavo::BitvavoClient;
use std::thread;
use std::time::Duration;
use log::info;

use serve::server::{PublishRequest, PublishResponse, SubscribeRequest, Message};
use serve::server::publisher_server::{PublisherServer, Publisher};

#[derive(Debug, Default)]
pub struct MyPublisher {}

#[tonic::async_trait]
impl Publisher for MyPublisher {

    async fn publish(&self, request: Request<PublishRequest>) -> Result<Response<PublishResponse>, Status> {
        info!("Got a request: {:?}", request);

        let reply = PublishResponse {
            message: format!("Hello {}!", request.into_inner().message).into(),
        };

        Ok(Response::new(reply))
    }

    type SubscribeStream = Pin<Box<dyn Stream<Item = Result<serve::server::Message, Status>> + Send + Sync + 'static>>;

    async fn subscribe(&self, request: Request<SubscribeRequest>) -> Result<Response<Self::SubscribeStream>, Status> {

        let bitvavo_client = BitvavoClient::default();

        let output = async_stream::try_stream! {
            loop {
                let book_result = bitvavo_client.get_book("BTC-EUR", 5).await;
                if(book_result.is_ok()) {
                    let book = book_result.expect("Failed to get book");

                    let book_message = serve::server::Message {
                        message: format!("{:?}", book),
                    };

                    info!("yielding: {:?}", book_message);

                    yield book_message
                }
                thread::sleep(Duration::from_secs(1));
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
    let publisher = MyPublisher::default();

    Server::builder()
        .add_service(PublisherServer::new(publisher))
        .serve(addr)
        .await?;

    Ok(())
}