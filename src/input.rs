use std::time::Duration;

use tokio::{sync::mpsc::Receiver, time::sleep};



pub fn input_task() -> Receiver<String> {

    // create mpcs channel queue
    let (sender,receiver) = tokio::sync::mpsc::channel::<String>(100);

    tokio::spawn(async move {
        let mut count = 0;
        loop {
            count += 1;
            let _ = sender.send(format!("compteur: {}", count)).await;
            println!("input compteur: {}", count);
            sleep(Duration::from_secs(1)).await;
        }
    });

    receiver
}