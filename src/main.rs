#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate thiserror;

mod hashes;
mod keys;
use keys::KeysCarousel;
mod virustotal;
mod queryer;
use queryer::Queryer;
mod requester;

#[tokio::main]
async fn main() {
    env_logger::init();

    let keys = KeysCarousel::from_file("keys.txt", 15).unwrap();
    let hashes = vec!["3395856ce81f2b7382dee72602f798b642f14140", "d235c2a0f84d55653fe91d9af7d804f5"];

    let queryer = Queryer::new(keys);
    for hash in hashes {
        let res = queryer.query(hash).await.unwrap();
        println!("{:?}", res);
    }
}
