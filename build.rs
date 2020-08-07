fn main() {
    capnpc::CompilerCommand::new()
        .src_prefix("protos")
        .file("protos/hash_query.capnp")
        .file("protos/response.capnp")
        .output_path("src")
        .run().expect("capnp schema compilation");
}
