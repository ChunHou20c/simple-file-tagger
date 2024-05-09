use hello_world::greeter_client::GreeterClient;
use hello_world::{HelloRequest,HelloNumber};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let request2 = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let request3 = tonic::Request::new(HelloNumber {
        message: 2,
    });

    let response = client.say_hello(request).await?;
    let response2 = client.say_bye(request2).await?;
    let response3 = client.say_repeat(request3).await?;

    println!("RESPONSE={:?}", response);
    println!("RESPONSE2={:?}", response2);
    println!("RESPONSE3={:?}", response3);

    Ok(())
}

