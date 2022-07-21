use tonic::{transport::Server, Request, Response, Status};

pub mod hello_world {
    include!(concat!(env!("OUT_DIR"), "/json.helloworld.Greeter.rs"));
}
use hello_world::greeter_server::{Greeter, GreeterServer};
use tonic_flatbuffers_example::common::{GreetRequestOwned, GreetResponseOwned};

#[derive(Debug, Clone, Copy, Default)]
pub struct MyGreeter {
    // This would be where we would store server state, such as references to different components and stuff.
}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<GreetRequestOwned>,
    ) -> Result<Response<GreetResponseOwned>, Status> {
        let username = request.get_ref().name().unwrap();
        println!("got request from user: {}", username);
        return Ok(Response::new(GreetResponseOwned::new(format!(
            "Hello, {}",
            username
        ))));
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
