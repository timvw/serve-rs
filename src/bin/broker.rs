use tonic::transport::Server;

use serve::broker::broker_server::{BrokerServer};
use serve::brokerimpl::{BrokerImpl};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"));

    let addr = "0.0.0.0:50051".parse()?;
    let broker = BrokerImpl::new();

    Server::builder()
        .add_service(BrokerServer::new(broker))
        .serve(addr)
        .await?;

    Ok(())
}