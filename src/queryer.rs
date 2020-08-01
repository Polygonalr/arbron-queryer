use crate::keys::KeysCarousel;
use crate::requester::{ Requester, RequestError };
use crate::requester::HashResult;

#[derive(Debug, Clone, Error)]
pub enum QueryError {
    #[error("error retrieving api key")]
    Keys,
    #[error("no api key could be retrieved")]
    NoKey,
    #[error("{0}")]
    Request(#[from] RequestError),
}

pub struct Queryer {
    keys: KeysCarousel,
    requester: Requester,
}
impl Queryer {
    pub fn new(keys:KeysCarousel) -> Self {
        Self {
            keys,
            requester: Requester::new(),
        }
    }

    pub async fn query(&self, hash:&str) -> Result<HashResult, QueryError> {
        let key = match self.keys.get_key().map_err(|_| QueryError::Keys)?.await {
            Some(key) => key,
            None => return Err(QueryError::NoKey),
        };
        self.requester.query(&key, hash).await.map_err(|e| QueryError::Request(e))
    }
}
