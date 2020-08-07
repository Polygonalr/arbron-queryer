use std::env::{ self, VarError };
use std::net::{ ToSocketAddrs, SocketAddr };

pub struct Config {
    pub keys_path: String,
    pub listen: SocketAddr,
    pub timeout: u64,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            keys_path: String::from("keys.txt"),
            listen: "0.0.0.0:1539".to_socket_addrs().unwrap().next().unwrap(),
            timeout: 15,
        }
    }
}
impl Config {
    pub fn load() -> Self {
        let mut config = Self::default();

        if let Some(keys_path) = load_env("ARBRON_QUERYER_KEYS_FILE") {
            config.keys_path = keys_path;
        }

        if let Some(listen) = load_env("ARBRON_QUERYER_LISTEN") {
            match to_addr(&listen) {
                Ok(listen) => config.listen = listen,
                Err(e) => warn!("{}", e),
            };
        }

        if let Some(timeout) = load_env("ARBRON_QUERYER_TIMEOUT") {
            match timeout.parse::<u64>() {
                Ok(timeout) => config.timeout = timeout,
                Err(_) => warn!("provided timeout \"{}\" was not a u64, using default of {}", timeout, config.timeout),
            };
        }

        config
    }
}

fn load_env(name:&str) -> Option<String> {
    match env::var(name) {
        Ok(val) => Some(val),
        Err(VarError::NotPresent) => None,
        Err(VarError::NotUnicode(_)) => {
            debug!("env {} is not unicode", name);
            None
        },
    }
}

fn to_addr(raw:&str) -> Result<SocketAddr, String> {
    raw.to_socket_addrs()
        .map_err(|e| format!("error parsing address: {}", e))?
        .next()
        .map_or(Err(String::from("no address found in provided raw")), |addr| Ok(addr))
}
