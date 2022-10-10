pub mod lsp_parser;
pub mod server;
pub mod util;

use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use futures_util::TryFutureExt;
use server::lsp_user::LSPParsers;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use tokio::sync::Mutex;
use tokio_stream::wrappers::UnboundedReceiverStream;

use server::handler::lsp_handler;
use tokio::sync::mpsc;
use warp::{
    ws::{Message, WebSocket},
    Filter,
};

#[derive(Clone)]
pub struct UserData {
    pub tx: mpsc::UnboundedSender<Message>,
    pub parsers: LSPParsers,
}

impl UserData {
    pub fn new(tx: mpsc::UnboundedSender<Message>) -> Self {
        Self {
            tx,
            parsers: LSPParsers::default(),
        }
    }
}

type Users = Arc<Mutex<HashMap<usize, UserData>>>;

/// Our global unique user id counter.
static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

#[tokio::main]
async fn main() {
    let users = Users::default();
    let users = warp::any().map(move || users.clone());

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let handler = warp::path!("laze_lsp")
        .and(warp::ws())
        .and(users)
        .map(|ws: warp::ws::Ws, users| ws.on_upgrade(move |socket| user_connected(socket, users)));

    warp::serve(handler).run(([127, 0, 0, 1], 3000)).await;
}

async fn user_connected(ws: WebSocket, users: Users) {
    // Use a counter to assign a new unique ID for this user.
    let my_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);

    eprintln!("new lsp user: {}", my_id);

    // Split the socket into a sender and receive of messages.
    let (mut user_ws_tx, mut user_ws_rx) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages
    // to the websocket...
    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);

    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            user_ws_tx
                .send(message)
                .unwrap_or_else(|e| {
                    eprintln!("websocket send error: {}", e);
                })
                .await;
        }
    });

    // Save the sender in our list of connected users.
    users.lock().await.insert(my_id, UserData::new(tx));

    // Return a `Future` that is basically a state machine managing
    // this specific user's connection.

    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error(uid={}): {}", my_id, e);
                break;
            }
        };
        let command = if let Ok(s) = msg.to_str() {
            s
        } else {
            return;
        };
        lsp_handler(
            command,
            users
                .lock()
                .await
                .get_mut(&my_id)
                .expect(format!("tx exists for {:?}", my_id).as_str()),
        )
        .await;
    }

    // user_ws_rx stream will keep processing as long as the user stays
    // connected. Once they disconnect, then...
    user_disconnected(my_id, &users).await;
}

async fn user_disconnected(my_id: usize, users: &Users) {
    eprintln!("good bye user: {}", my_id);

    // Stream closed up, so remove from the user list
    users.lock().await.remove(&my_id);
}
