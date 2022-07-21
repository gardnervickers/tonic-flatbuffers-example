// The hello_generated module is generated using the flatbuffers compiler. You can find the command in the `gen.sh`
// script at the project root.
#[allow(dead_code, unused_imports)]
#[path = "../proto/hello_generated.rs"]
pub mod hello_generated;

pub mod codec;

pub mod hello_world {
    include!(concat!(env!("OUT_DIR"), "/json.helloworld.Greeter.rs"));
}
use hello_world::greeter_client::GreeterClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = GreeterClient::connect("http://[::1]:50051").await?;

    //let request = tonic::Request::new(HelloRequest {
    //    name: "Tonic".into(),
    //});

    //let response = client.say_hello(request).await?;

    Ok(())
}
