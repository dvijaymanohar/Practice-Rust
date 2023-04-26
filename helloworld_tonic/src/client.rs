use tonic::transport::Endpoint;
use tonic::Request;

use hello::hello_client::HelloClient;
use hello::HelloRequest;
use learning_grpc::hello;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let addr = Endpoint::from_static("https://127.0.0.1:50051");
    let addr = Endpoint::from_static("https://172.16.159.131:50052");

    let mut client = HelloClient::connect(addr).await?;
    let request = Request::new(HelloRequest {});
    let response = client.hello_world(request).await?;
    println!("response: {}", response.into_inner().message);

    Ok(())
}
