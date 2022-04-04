use serve::server::SubscribeRequest;
use serve::server::publisher_client::PublisherClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut publisher_client = PublisherClient::connect("http://[::1]:50051").await?;

    let mut stream = publisher_client.subscribe(SubscribeRequest{})
        .await?
        .into_inner();

    while let Some(msg) = stream.message().await? {
        println!("msg = {:?}", msg);
    }

    Ok(())
}