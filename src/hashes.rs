use regex::Regex;

pub enum HashType {
    Md5,
    Sha1,
    Sha256,
    Unknown,
}

pub fn categorise_hash(hash:&str) -> HashType {
    lazy_static! {
        static ref MD5_RE:Regex = Regex::new("^[0-9a-fA-F]{32}$").unwrap();
        static ref SHA1_RE:Regex = Regex::new("^[0-9a-fA-F]{40}$").unwrap();
        static ref SHA256_RE:Regex = Regex::new("^[0-9a-fA-F]{64}$").unwrap();
    }

    let hash = hash.trim();

    if MD5_RE.is_match(hash) {
        HashType::Md5
    } else if SHA1_RE.is_match(hash) {
        HashType::Sha1
    } else if SHA256_RE.is_match(hash) {
        HashType::Sha256
    } else {
        HashType::Unknown
    }
}
