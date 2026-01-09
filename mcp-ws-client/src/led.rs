use std::error::Error;
use std::sync::Arc;
use rmcp::handler::server::tool::ToolRouter;
use rmcp::{tool, tool_handler, tool_router, ErrorData, ServerHandler, ServiceExt};
use rmcp::model::{CallToolResult, Content, ErrorCode, ServerCapabilities, ServerInfo};
use rmcp::service::{RunningService, ServerInitializeError, ServiceRole};
use rmcp::transport::stdio;
use rust_socketio::asynchronous::Client;

pub async fn initialize(socket: &Client) -> Result<(), Box<dyn Error>> {
    let socket = Arc::new(socket.clone());
    let led_server = LedServer::new(socket.clone());

    let mcp_led_service = led_server
        .serve(stdio())
        .await;


    mcp_led_service
        .inspect_err(|err| println!("{:?}", err))?
        .waiting()
        .await?;

    Ok(())
}

struct LedServer {
    socket: Arc<Client>,
    tool_router: ToolRouter<Self>
}

#[tool_router]
impl LedServer {
    pub fn new(socket: Arc<Client>) -> Self {
        Self {
            socket,
            tool_router: Self::tool_router()
        }
    }

    #[tool(description = "Turn the LED on; set the LED to high.")]
    async fn high(&self) -> Result<CallToolResult, ErrorData> {
        let broadcast_error = self.socket.emit("server_high", "").await.err();

        if broadcast_error.is_some() {
            return Err(
                ErrorData::new(
                    ErrorCode::INTERNAL_ERROR,
                    "Websocket broadcast error.",
                    None
                )
            );
        }

        Ok(
            CallToolResult::success(
                vec![Content::text("Set LED to HIGH.")]
            )
        )
    }

    #[tool(description = "Turn the LED off; set the LED to low.")]
    async fn low(&self) -> Result<CallToolResult, ErrorData> {
        let broadcast_error = self.socket.emit("server_low", "").await.err();

        if broadcast_error.is_some() {
            return Err(
                ErrorData::new(
                    ErrorCode::INTERNAL_ERROR,
                    "Websocket broadcast error.",
                    None
                )
            );
        }

        Ok(
            CallToolResult::success(
                vec![Content::text("Set LED to LOW.")]
            )
        )
    }
}

#[tool_handler]
impl ServerHandler for LedServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("Turn a LED on or off (high / low)".to_string()),
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            ..Default::default()
        }
    }
}