use serve::broker::{PublishRequest};
use serve::broker::broker_client::BrokerClient;
use chrono::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    loop {

        let mut broker_client = BrokerClient::connect("http://[::1]:50051").await?;

        let request = tonic::Request::new(PublishRequest {
            message: format!("The time is {:?}", Utc::now()),
        });

        let response = broker_client.publish(request).await?;
        println!("RESPONSE={:?}", response);
    }

    Ok(())
}
