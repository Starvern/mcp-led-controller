use std::sync::Arc;
use rust_socketio::asynchronous::ClientBuilder;

mod led;

#[tokio::main]
async fn main() {
    let client = ClientBuilder::new("http://192.168.1.132:3000/")
        .namespace("/")
        .connect()
        .await
        .expect("Connection failed.");

    led::initialize(&client).await.expect("Failed to init MCP server.");

    client.disconnect().await.expect("Disconnect failed.");
}
