mod sse;
mod input;
mod business;

use business::{Publisher, Transformer, UpperCaseTransformer};
use input::input_task;

use tokio::time::sleep;
use tokio::sync::mpsc;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {

    let (broadcast, _) = broadcast::channel::<String>(100);

    
    let transformer = Arc::new(UpperCaseTransformer);
    
    let input_receiver = input_task();

    let sse = Arc::new( sse::SseHandler::new(broadcast.clone()));
    
     business_logic_task(input_receiver, transformer,sse.clone());
    
    
    
    sse.sse_task();

    loop {
        sleep(Duration::from_secs(1)).await;
    }
}

fn business_logic_task(mut input: mpsc::Receiver<String>, transformer: Arc<dyn Transformer>,publisher: Arc<dyn Publisher>)  {

    

    tokio::spawn(async move {

        let dispatcher = business::Dispatcher::new(
            publisher,       
        );

        while let Some(input) = input.recv().await {
            let transformed = transformer.transform(&input);
            dispatcher.dispatch(transformed); 
        }
    });

}
