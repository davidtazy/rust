use axum::{
    Router,
    extract::State,
    response::sse::{Event, KeepAlive, Sse},
    routing::get,
};

use futures::stream;
use futures::stream::Stream;
use std::convert::Infallible;
use tokio::net::TcpListener;
use axum::serve;
use tokio::sync::broadcast::Sender;
use std::net::SocketAddr;

use crate::business::Publisher;

pub struct SseHandler{
     sender: Sender<String>,
}

impl SseHandler {
    pub fn new(sender: Sender<String>) -> Self {
        SseHandler { sender }
    }

    pub fn sse_task(&self) -> tokio::task::JoinHandle<()> {

        let subscriber = self.sender.clone();
        
        tokio::spawn(async move {
            let app = app(subscriber);
    
            // Création du listener TCP
            let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
            let listener = TcpListener::bind(addr).await.unwrap();
            println!("Serveur en écoute sur http://{}", addr);
    
            // Serveur avec axum::serve
            serve(listener, app).await.unwrap();
        })
    }
}

impl Publisher for SseHandler {
    fn publish(&self, message: String) {
        // Envoi du message à tous les abonnés
        let _ = self.sender.send(message);
    }
}




#[derive(Clone)]
pub struct AppState {
    pub tx: Sender<String>,
}

pub fn app(tx: Sender<String>) -> Router {
    let state = AppState { tx };
    Router::new()
        .route("/events", get(sse_handler))
        .with_state(state)
}

async fn sse_handler(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let  rx = state.tx.subscribe();

    let stream = stream::unfold(rx, |mut rx| async {
        match rx.recv().await {
            Ok(msg) => Some((Ok(Event::default().data(msg)), rx)),
            Err(_) => None,
        }
    });

    Sse::new(stream).keep_alive(KeepAlive::default())
}
