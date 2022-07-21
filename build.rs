fn main() {
    // We have to describe routes here using the builder, rather than generating them from some RPC file.
    let greeter_service = tonic_build::manual::Service::builder()
        .name("Greeter")
        .package("json.helloworld")
        .method(
            tonic_build::manual::Method::builder()
                .name("say_hello")
                .route_name("SayHello")
                .input_type("tonic_flatbuffers_example::common::GreetRequestOwned")
                .output_type("tonic_flatbuffers_example::common::GreetResponseOwned")
                .codec_path("tonic_flatbuffers_example::codec::FlatbuffersCodec")
                .build(),
        )
        .build();
    tonic_build::manual::Builder::new().compile(&[greeter_service]);
}
