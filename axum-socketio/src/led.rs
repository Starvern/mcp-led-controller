use std::error::Error;
use rmcp::handler::server::tool::ToolRouter;
use rmcp::{tool, tool_handler, tool_router, ErrorData, ServerHandler, ServiceExt};
use rmcp::model::{CallToolResult, Content, ErrorCode, ServerCapabilities, ServerInfo};
use rmcp::transport::stdio;
use socketioxide::SocketIo;
use tracing::log::warn;

pub async fn initialize(io: SocketIo) -> Result<(), Box<dyn Error>> {
    let mcp_led_service = LedServer::new(io)
        .serve(stdio())
        .await
        .inspect_err(|err| warn!("{:?}", err))?;

    mcp_led_service.waiting().await?;

    Ok(())
}

struct LedServer {
    io: SocketIo,
    tool_router: ToolRouter<Self>
}

#[tool_router]
impl LedServer {

    pub fn new(io: SocketIo) -> Self {
        Self {
            io,
            tool_router: Self::tool_router()
        }
    }

    #[tool(description = "Turn the LED on; set the LED to high.")]
    async fn high(&self) -> Result<CallToolResult, ErrorData> {
        let broadcast_error = self.io.emit("high", &()).await.err();

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
        let broadcast_error = self.io.emit("low", &()).await.err();

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