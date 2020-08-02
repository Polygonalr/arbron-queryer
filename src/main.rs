#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate thiserror;

mod hashes;
#[allow(dead_code)]
mod hash_query_capnp;
use hash_query_capnp::hash_query::Client;
mod keys;
use keys::KeysCarousel;
mod virustotal;
mod queryer;
use queryer::Queryer;
mod requester;
mod rpc;
use rpc::HashQueryServer;

use async_std::net::TcpListener;
use futures::{ AsyncReadExt, FutureExt };
use std::net::ToSocketAddrs;
use capnp_rpc::RpcSystem;
use capnp_rpc::rpc_twoparty_capnp::Side;
use capnp_rpc::twoparty::VatNetwork;

#[tokio::main]
async fn main() {
    env_logger::init();

    let keys = KeysCarousel::from_file("keys.txt", 15).unwrap();
    let server = HashQueryServer::new(Queryer::new(keys));

    let addr = ":1539".to_socket_addrs().unwrap().next().unwrap();
    async_std::task::block_on(async move {
        let listener = TcpListener::bind(&addr).await.unwrap();
        let client:Client = capnp_rpc::new_client(server);

        loop {
            let (stream, _) = listener.accept().await.unwrap();
            stream.set_nodelay(true).unwrap();
            let (reader, writer) = stream.split();
            let network = VatNetwork::new(reader, writer, Side::Server, Default::default());

            let rpc_system = RpcSystem::new(Box::new(network), Some(client.clone().client));
            async_std::task::spawn_local(rpc_system.map(|_| ()));
        }
    });
}
