use serve::broker::{PublishRequest};
use serve::broker::broker_client::BrokerClient;
use std::{thread, time};
use std::time::Duration;
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

        //thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}
