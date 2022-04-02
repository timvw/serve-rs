use tonic::{transport::Server, Request, Response, Status};

use serve::server::{PublishRequest, PublishResponse};
use serve::server::publisher_server::{PublisherServer, Publisher};

#[derive(Debug, Default)]
pub struct MyPublisher {}

#[tonic::async_trait]
impl Publisher for MyPublisher {
    async fn publish(&self, request: Request<PublishRequest>) -> Result<Response<PublishResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = PublishResponse {
            message: format!("Hello {}!", request.into_inner().message).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let publisher = MyPublisher::default();

    Server::builder()
        .add_service(PublisherServer::new(publisher))
        .serve(addr)
        .await?;

    Ok(())
}