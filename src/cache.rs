use crate::business::Repository;
use futures::channel::oneshot::Receiver;
use mini_redis::{Result};

use bytes::Bytes;
use tokio::sync::oneshot;

pub struct Cache {
    sender: tokio::sync::mpsc::Sender<Command>,
}

#[async_trait::async_trait]
impl Repository for Cache {
    async fn save(&self, message: String) -> std::result::Result<(), String> {
       
        let (resp_sender, resp_receiver) = oneshot::channel();

        let key = message.clone();
        let value = Bytes::from(message.clone());
        let val = value.clone();

        let command = Command::Set { key, val, resp: resp_sender };
        self.sender.send(command).await.unwrap();
        if resp_receiver.await.is_ok() {
            Ok(())
        } else {
            Err("Failed to save message".to_string())
        }

    }

    async fn get(&self, key: String) -> std::result::Result<String, String> {
        let (resp_sender, resp_receiver) = oneshot::channel();

        let command = Command::Get { key, resp: resp_sender };
        self.sender.send(command).await.unwrap();

        match resp_receiver.await {
            Ok(result) => match result {
                Ok(value) => Ok(String::from_utf8(value.unwrap().to_vec()).unwrap()),
                Err(_) => Err("Failed to get message".to_string()),
            },
            Err(_) => Err("Failed to receive response".to_string()),
        }
    }
    
}

impl Cache {
    pub fn new(sender: tokio::sync::mpsc::Sender<Command>) -> Self {
        Cache { sender }
    }

    pub fn task(&self,mut receiver: tokio::sync::mpsc::Receiver<Command>) {
       
        tokio::spawn(async move {
            while let Some(command) = receiver.recv().await {
                match command {
                    Command::Get { key, resp } => {
                        // Simulate a cache hit
                        let value = Some(Bytes::from(format!("Value for {}", key)));
                        let _ = resp.send(Ok(value));
                    }
                    Command::Set { key, val, resp } => {
                        // Simulate a cache set
                        println!("Setting {} to {:?}", key, val);
                        let _ = resp.send(Ok(()));
                    }
                }
            }
        });
    }
}

#[derive(Debug)]
pub enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;
