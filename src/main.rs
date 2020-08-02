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
use std::net::{ ToSocketAddrs, SocketAddr };
use capnp_rpc::RpcSystem;
use capnp_rpc::rpc_twoparty_capnp::Side;
use capnp_rpc::twoparty::VatNetwork;
use tokio::task::LocalSet;

#[tokio::main]
async fn main() {
    env_logger::init();

    let keys = match KeysCarousel::from_file("keys.txt", 15) {
        Ok(keys) => keys,
        Err(e) => {
            error!("error reading keys file: {}", e);
            return;
        },
    };
    let server = HashQueryServer::new(Queryer::new(keys));

    let addr = match to_addr("0.0.0.0:1539") {
        Ok(addr) => addr,
        Err(e) => {
            error!("{}", e);
            return;
        },
    };

    let localset = LocalSet::new();
    localset.run_until(async move {
        let listener = match TcpListener::bind(&addr).await {
            Ok(listener) => listener,
            Err(e) => {
                error!("cannot bind listener: {}", e);
                return;
            },
        };

        let client:Client = capnp_rpc::new_client(server);

        info!("listening on {}", addr);
        loop {
            let stream = match listener.accept().await {
                Ok((stream, _)) => stream,
                Err(e) => {
                    warn!("unable to accept a connection: {}", e);
                    continue;
                },
            };

            if let Err(e) = stream.set_nodelay(true) {
                warn!("unable to set nodelay: {}", e);
                continue;
            }

            let (reader, writer) = stream.split();
            let network = VatNetwork::new(reader, writer, Side::Server, Default::default());

            let rpc_system = RpcSystem::new(Box::new(network), Some(client.clone().client));
            tokio::task::spawn_local(rpc_system.map(|_| ()));
        }
    }).await;
}

fn to_addr(raw:&str) -> Result<SocketAddr, String> {
    raw.to_socket_addrs()
        .map_err(|e| format!("error parsing address: {}", e))?
        .next()
        .map_or(Err(String::from("no address found in provided raw")), |addr| Ok(addr))
}
