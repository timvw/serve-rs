use serve::broker::{PublishRequest};
use serve::broker::broker_client::BrokerClient;
use serve::bitvavo::BitvavoClient;
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let bitvavo_client = BitvavoClient::default();
    let mut broker_client = BrokerClient::connect("http://[::1]:50051").await?;

    loop {

        let book = bitvavo_client.get_book("BTC-EUR", 5).await?;

        let request = tonic::Request::new(PublishRequest {
            message: format!("{:?}", book).into(),
        });

        let response = broker_client.publish(request).await?;
        println!("RESPONSE={:?}", response);

        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}
