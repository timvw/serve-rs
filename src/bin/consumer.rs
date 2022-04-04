use serve::broker::SubscribeRequest;
use serve::broker::broker_client::BrokerClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut broker_client = BrokerClient::connect("http://[::1]:50051").await?;

    let mut stream = broker_client.subscribe(SubscribeRequest{})
        .await?
        .into_inner();

    while let Some(msg) = stream.message().await? {
        println!("msg = {:?}", msg);
    }

    Ok(())
}