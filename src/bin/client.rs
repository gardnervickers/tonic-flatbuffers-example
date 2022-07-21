pub mod hello_world {
    include!(concat!(env!("OUT_DIR"), "/json.helloworld.Greeter.rs"));
}
use hello_world::greeter_client::GreeterClient;
use tonic_flatbuffers_example::common::GreetRequestOwned;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;
    let request = tonic::Request::new(GreetRequestOwned::new("gardner"));
    let response = client.say_hello(request).await?;
    println!(
        "response received: {}",
        response.get_ref().message().unwrap()
    );
    Ok(())
}
