use futures::Future;
use futures::task::{ Context, Poll, Waker };
use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::pin::Pin;
use std::sync::{ Arc, Mutex };
use std::sync::mpsc::{ self, Receiver, Sender };
use std::time::Duration;

pub struct KeyResult {
    key: Arc<Mutex<Option<Option<String>>>>,
    waker: Arc<Mutex<Option<Waker>>>,
}
impl KeyResult {
    pub fn new(rx:Receiver<String>) -> Self {
        let key_result = KeyResult {
            key: Arc::new(Mutex::new(None)),
            waker: Arc::new(Mutex::new(None)),
        };

        let key = key_result.key.clone();
        let waker = key_result.waker.clone();
        std::thread::spawn(move || {
            let res = rx.recv();

            let mut key = key.lock().unwrap();
            *key = Some(res.ok());

            let mut waker = waker.lock().unwrap();
            if let Some(waker) = waker.take() {
                waker.wake();
            }
        });

        key_result
    }
}
impl Future for KeyResult {
    type Output = Option<String>;
    fn poll(self:Pin<&mut Self>, ctx:&mut Context) -> Poll<Self::Output> {
        let key = self.key.lock().unwrap();
        if let Some(res) = key.clone() {
            Poll::Ready(res.clone())
        } else {
            let mut waker = self.waker.lock().unwrap();
            *waker = Some(ctx.waker().clone());
            Poll::Pending
        }
    }
}

pub struct KeysCarousel {
    requests: Sender<Sender<String>>,
}
impl KeysCarousel {
    pub fn from_file(filename:&str, timeout:u64) -> Result<Self, std::io::Error> {
        let mut file = File::open(Path::new(filename))?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;

        let re = Regex::new("[0-9a-fA-F]+").unwrap();
        let keys = s.split('\n').map(|k| k.trim().to_string()).filter(|k| re.is_match(k)).collect::<Vec<String>>();

        Ok(Self::new(keys, timeout))
    }

    pub fn new(keys:Vec<String>, timeout:u64) -> Self {
        let (key_tx, key_rx) = mpsc::channel();
        for key in keys.iter() {
            Self::spawn_key(key, timeout, key_tx.clone());
        }

        let (req_tx, req_rx):(Sender<Sender<String>>, Receiver<Sender<String>>) = mpsc::channel();

        std::thread::spawn(move || {
            loop {
                if let Ok(req) = req_rx.recv() {
                    if let Ok(key) = key_rx.recv() {
                        if let Err(_) = req.send(key) {
                            warn!("request receiver has been dropped");
                        }
                    } else {
                        error!("all key transmitters have been dropped");
                        break;
                    }
                } else {
                    error!("request transmitter has been dropped");
                    break;
                }
            }
        });

        Self {
            requests: req_tx,
        }
    }

    pub fn get_key(&self) -> Result<KeyResult, ()> {
        let (tx, rx) = mpsc::channel();
        let res = KeyResult::new(rx);
        self.requests.send(tx).map_err(|_| ())?;
        Ok(res)
    }

    fn spawn_key(key:&str, max_timeout:u64, tx:Sender<String>) {
        let key = key.to_string();
        std::thread::spawn(move || {
            let mut current_timeout = max_timeout;
            loop {
                if current_timeout < max_timeout {
                    current_timeout += 1;
                } else {
                    current_timeout = 0;
                    if let Err(_) = tx.send(key.clone()) {
                        error!("key receiver in key carousel has been dropped");
                        break;
                    }
                }
                std::thread::sleep(Duration::from_secs(1));
            }
        });
    }
}
