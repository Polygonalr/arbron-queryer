#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate thiserror;

mod hashes;
mod virustotal;
mod requester;
use requester::Queryer;

#[tokio::main]
async fn main() {
    let api_key = "";
    let hash = "d235c2a0f84d55653fe91d9af7d804f5";

    let queryer = Queryer::new();
    let res = queryer.query(api_key, hash).await.unwrap();
    println!("{:?}", res);
}
