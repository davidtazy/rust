mod business;
mod cache;
mod input;
mod sse;

use business::{Publisher, Transformer, UpperCaseTransformer};
use cache::Command;
use cache::Cache;
use input::input_task;

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    
    let (cache_sender, cache_receiver) = tokio::sync::mpsc::channel::<Command>(100);

    let transformer = Arc::new(UpperCaseTransformer);

    let input_receiver = input_task();

    let cache = Arc::new(Cache::new(cache_sender));
    let sse = Arc::new(sse::SseHandler::new());

    cache.task(cache_receiver);

    business_logic_task(input_receiver, transformer, sse.clone(),cache.clone());

    sse.sse_task();

    loop {
        sleep(Duration::from_secs(1)).await;
    }
}

fn business_logic_task(
    mut input: mpsc::Receiver<String>,
    transformer: Arc<dyn Transformer>,
    publisher: Arc<dyn Publisher>,
    cache: Arc<dyn business::Repository>,
) {
    tokio::spawn(async move {
        let dispatcher = business::Dispatcher::new(publisher, cache);

        while let Some(input) = input.recv().await {
            let transformed = transformer.transform(&input);
            dispatcher.dispatch(transformed).await;
        }
    });
}
