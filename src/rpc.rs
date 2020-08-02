use capnp::capability::Promise;
use crate::hash_query_capnp::hash_query::{ Server, QueryParams, QueryResults };
use crate::queryer::Queryer;
use crate::hashes::{ self, HashType };
use std::sync::Arc;

pub struct HashQueryServer {
    queryer: Arc<Queryer>,
}
impl HashQueryServer {
    pub fn new(queryer:Queryer) -> Self {
        Self {
            queryer: Arc::new(queryer),
        }
    }

    pub async fn use_queryer(queryer:Arc<Queryer>, params:QueryParams, mut res:QueryResults) -> Result<(), capnp::Error> {
        let request = params.get()?.get_req()?;
        let hash = request.get_hash()?;
        let hash_result = queryer.query(hash).await.map_err(|e| capnp::Error::failed(format!("{}", e)))?;

        let mut res = res.get().init_res();
        res.set_hash(&hash_result.hash);
        res.set_detected(hash_result.detected);

        match hashes::categorise_hash(hash) {
            HashType::Md5 => res.set_md5(()),
            _ => {
                let mut translation = res.init_translation();
                translation.set_original(hash);
                translation.set_md5(&hash_result.md5);
            },
        };

        Ok(())
    }
}
impl Server for HashQueryServer {
    fn query(&mut self, params:QueryParams, results:QueryResults) -> Promise<(), capnp::Error> {
        Promise::from_future(Self::use_queryer(self.queryer.clone(), params, results))
    }
}