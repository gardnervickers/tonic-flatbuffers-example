use codec::FlatbuffersMessage;
use tonic::{transport::Server, Request, Response, Status};

// The hello_generated module is generated using the flatbuffers compiler. You can find the command in the `gen.sh`
// script at the project root.
#[allow(dead_code, unused_imports)]
#[path = "../proto/hello_generated.rs"]
pub(crate) mod hello_generated;

pub mod codec;

pub mod hello_world {
    include!(concat!(env!("OUT_DIR"), "/json.helloworld.Greeter.rs"));
}
use hello_world::greeter_server::{Greeter, GreeterServer};

#[derive(Debug, Clone, Copy, Default)]
pub struct MyGreeter {
    // This would be where we would store server state, such as references to different components and stuff.
}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<FlatbuffersMessage>,
    ) -> Result<Response<FlatbuffersMessage>, Status> {
        todo!("do async stuff to respond to the request")
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
