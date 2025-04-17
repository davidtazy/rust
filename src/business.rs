use std::sync::Arc;

use async_trait::async_trait;

pub trait Transformer: Send + Sync {
    fn transform(&self, input: &str) -> String;
}

pub trait Publisher: Send + Sync {
    fn publish(&self, message: String);
}

#[async_trait]
pub trait Repository : Send + Sync {
    async fn save(&self, message: String) -> Result<(), String>;
    async fn get(&self, key: String) -> Result<String, String>;
}

pub struct UpperCaseTransformer;
impl Transformer for UpperCaseTransformer {
    fn transform(&self, input: &str) -> String {
        input.to_uppercase()
    }
}

pub struct Dispatcher {
    publisher: Arc<dyn Publisher>,
    repository: Arc<dyn Repository>,
}

impl Dispatcher {
    pub fn new(publisher: Arc<dyn Publisher>  , repository: Arc<dyn Repository>) -> Self {
        Dispatcher {
            publisher, repository,
        }
    }

    pub async fn dispatch(&self, message: String) {
        self.publisher.publish(message.clone());
        self.repository.save(message).await.unwrap();
    }
}
