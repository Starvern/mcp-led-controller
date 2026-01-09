use axum::routing::get;
use socketioxide::{SocketIo};
use socketioxide::extract::{Data, SocketRef};
use tracing::{info, warn};
use tracing_subscriber::FmtSubscriber;

async fn on_message(socket: SocketRef, Data(data): Data<String>) {
    info!("Event 'message' received data: {:?}", data);
    info!("Replying back 'message' with data: 'hello from socket server!'.");

    let res = socket.emit("message", "Hello from socket server!");

    if res.is_err() {
        warn!("Failed to emit message from server!");
    }
}

async fn on_high(socket: SocketRef) {
    info!("Receiving signal to activate LED on HIGH.");
    info!("Dispatching event 'high' to all clients.");

    let res = socket.broadcast().emit("high", "high").await;
    if res.is_err() {
        warn!("Failed to emit event 'high' to all clients.");
    }
}

async fn on_low(socket: SocketRef) {
    info!("Receiving signal to activate LED on LOW.");
    info!("Dispatching event 'low' to all clients.");

    let res = socket.broadcast().emit("low", "low").await;
    if res.is_err() {
        warn!("Failed to emit event 'low' to all clients.");
    }
}

async fn on_connect(socket: SocketRef) {
    info!("Socket.io connected: {:?} {:?}", socket.ns(), socket.id);

    socket.on("message", on_message);
    socket.on("server_high", on_high);
    socket.on("server_low", on_low);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;
    info!("[SERVER] Initializing...");

    let (layer, io) = SocketIo::new_layer();

    io.ns("/", on_connect);

    let app = axum::Router::new()
        .route("/", get(|| async { "From '/' get handler!" }))
        .layer(layer);

    info!("[SERVER] Done!");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;



    Ok(())
}

