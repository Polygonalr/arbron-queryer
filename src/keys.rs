use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{ AtomicU64, Ordering };
use std::time::Duration;

struct KeyHolder {
    key: String,
    timeout: u64,
    current_timeout: AtomicU64,
}
impl KeyHolder {
    pub fn new(key:String, timeout:u64) -> Self {
        Self {
            key,
            timeout,
            current_timeout: AtomicU64::new(timeout),
        }
    }

    pub fn get_key(&self) -> Option<&str> {
        if self.current_timeout.load(Ordering::Acquire) >= self.timeout {
            self.current_timeout.swap(0, Ordering::Release);
            Some(&self.key)
        } else {
            None
        }
    }

    pub fn tick(&self) {
        if self.current_timeout.load(Ordering::Acquire) < self.timeout {
            let current_timeout = self.current_timeout.load(Ordering::Acquire);
            self.current_timeout.swap(current_timeout + 1, Ordering::Release);
        }
    }
}

pub struct KeysCarousel {
    keys: Vec<KeyHolder>,
}
impl KeysCarousel {
    pub fn from_file(filename:&str, timeout:u64) -> Result<Arc<Self>, std::io::Error> {
        let mut file = File::open(Path::new(filename))?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;

        let re = Regex::new("[0-9a-fA-F]+").unwrap();
        let keys = s.split('\n').map(|k| k.trim().to_string()).filter(|k| re.is_match(k)).collect::<Vec<String>>();

        Ok(Self::new(keys, timeout))
    }

    pub fn new(keys:Vec<String>, timeout:u64) -> Arc<Self> {
        let keys = keys.iter().map(|key| KeyHolder::new(key.to_string(), timeout)).collect::<Vec<KeyHolder>>();

        let kc = Arc::new(Self {
            keys,
        });

        {
            let kc = kc.clone();
            std::thread::spawn(move || {
                loop {
                    kc.tick();
                    std::thread::sleep(Duration::from_secs(1));
                }
            });
        }

        kc
    }

    pub fn get_key(&self) -> Option<&str> {
        self.keys.iter().filter_map(|k| k.get_key()).next()
    }

    pub fn tick(&self) {
        for key in &self.keys {
            key.tick();
        }
    }
}
