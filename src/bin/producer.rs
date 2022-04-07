use std::thread;
use std::time::Duration;
use serve::broker::{PublishRequest};
use serve::broker::broker_client::BrokerClient;
use chrono::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args: Vec<String> = std::env::args().collect();
    let name = args[1].clone();

    loop {

        let mut broker_client = BrokerClient::connect("http://[::1]:50051").await?;

        let request = tonic::Request::new(PublishRequest {
            message: format!("[{:?}] -> The time is {:?}", name, Utc::now()),
        });

        let response = broker_client.publish(request).await?;
        println!("RESPONSE={:?}", response);

        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}
