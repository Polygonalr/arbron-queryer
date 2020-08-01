use crate::hashes::{ self, HashType };
use crate::virustotal::VirustotalResponse;
use reqwest::{ Client, StatusCode };
use serde::{ Deserialize, Serialize };

const VIRUSTOTAL_FILES_URL:&'static str = "https://www.virustotal.com/api/v3/files";

#[derive(Debug, Clone, Error)]
pub enum RequestError {
    #[error("a fatal error occured requesting: {0}")]
    Fatal(String),
    #[error("a non-fatal error occured requesting: {0}")]
    NonFatal(String),
    #[error("rate limited of api key exceeded")]
    RateLimitExceeded,
}
impl RequestError {
    pub fn from_reqwest_error(e:&reqwest::Error) -> Self {
        let message = format!("{}", e);

        match e.status() {
            Some(StatusCode::TOO_MANY_REQUESTS)  => Self::RateLimitExceeded,
            Some(StatusCode::SERVICE_UNAVAILABLE) => Self::NonFatal(message),
            _ => Self::Fatal(message),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HashResult {
    hash: String,
    md5: String,
    detected: bool,
}
impl HashResult {
    pub fn new(hash:&str, md5:&str, category:&str) -> Self {
        let detected = category == "suspicious" || category == "malicious";

        Self {
            hash: hash.to_string(),
            md5: md5.to_string(),
            detected,
        }
    }

    pub fn not_found(hash:&str) -> Self {
        let hash = hash.to_string();
        let md5 = match  hashes::categorise_hash(&hash) {
            HashType::Md5 => hash.clone(),
            _ => String::new(),
        };

        Self {
            hash,
            md5,
            detected: false,
        }
    }
}

pub struct Requester {
    client: Client,
}
impl Requester {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn query(&self, api_key:&str, hash:&str) -> Result<HashResult, RequestError> {
        match self.send_query(api_key, hash).await {
            Ok(Some(res)) => match res.data.attributes.last_analysis_results.symantec {
                Some(symantec) => Ok(HashResult::new(hash, &res.data.attributes.md5, &symantec.category)),
                None => Ok(HashResult::not_found(hash)),
            },
            Ok(None) => Ok(HashResult::not_found(hash)),
            Err(e) => match e.status() {
                Some(StatusCode::NOT_FOUND) => Ok(HashResult::not_found(hash)),
                _ => return Err(RequestError::from_reqwest_error(&e)),
            },
        }
    }

    async fn send_query(&self, api_key:&str, hash:&str) -> Result<Option<VirustotalResponse>, reqwest::Error> {
        let res = self.client.get(&format!("{}/{}", VIRUSTOTAL_FILES_URL, hash)).header("x-apikey", api_key).send().await?;

        if res.status() == StatusCode::NOT_FOUND {
            Ok(None)
        } else {
            let res:VirustotalResponse = res.json().await?;
            Ok(Some(res))
        }
    }
}
