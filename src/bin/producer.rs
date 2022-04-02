use serve::server::{PublishRequest, PublishResponse};
use serve::server::publisher_client::PublisherClient;
use serve::bitvavo::BitvavoClient;
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let bitvavo_client = BitvavoClient::default();
    let mut publisher_client = PublisherClient::connect("http://[::1]:50051").await?;

    loop {

        let book = bitvavo_client.get_book("BTC-EUR", 5).await?;

        let request = tonic::Request::new(PublishRequest {
            message: format!("{:?}", book).into(),
        });

        let response = publisher_client.publish(request).await?;
        println!("RESPONSE={:?}", response);

        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}
