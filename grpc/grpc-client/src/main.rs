use tonic::Request;

use greeter::greeter_client::GreeterClient;
use greeter::HelloRequest;

pub mod greeter {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = Request::new(HelloRequest {
        name: "World".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response.into_inner());

    Ok(())
}
