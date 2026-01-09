use std::sync::Arc;
use rmcp::handler::server::tool::ToolRouter;
use rmcp::{tool, tool_handler, tool_router, ErrorData, ServerHandler, ServiceExt};
use rmcp::model::{CallToolResult, Content, ServerCapabilities, ServerInfo};
use rmcp::transport::stdio;
use tokio::sync::Mutex;
use tracing::info;

pub async fn initialize_mcp() -> Result<(), Box<dyn std::error::Error>> {
    info!("MCP initializing...");
    let mcp_service = Counter::new()
        .serve(stdio())
        .await
        .inspect_err(|err| eprintln!("{:?}", err))?;

    mcp_service.waiting().await?;
    info!("MCP initialization completed.");

    Ok(())
}

pub struct Counter {
    counter: Arc<Mutex<i32>>,
    tool_router: ToolRouter<Self>
}


#[tool_router]
impl Counter {
    pub fn new() -> Self {
        Self {
            counter: Arc::new(Mutex::new(0)),
            tool_router: Self::tool_router()
        }
    }

    #[tool(
        name = "increment",
        description = "Increment counter by 1."
    )]
    async fn increment(&self) -> Result<CallToolResult, ErrorData> {
        let mut counter = self.counter.lock().await;
        *counter += 1;
        Ok(
            CallToolResult::success(vec![Content::text(counter.to_string())])
        )
    }

    #[tool(
        name = "decrement",
        description = "Decrement counter by 1."
    )]
    async fn decrement(&self) -> Result<CallToolResult, ErrorData> {
        let mut counter = self.counter.lock().await;
        *counter -= 1;
        Ok(
            CallToolResult::success(vec![Content::text(counter.to_string())])
        )
    }

    #[tool(
        name = "get_counter",
        description = "Get counter."
    )]
    async fn get_value(&self) -> Result<CallToolResult, ErrorData> {
        let counter = self.counter.lock().await;
        Ok(
            CallToolResult::success(vec![Content::text(counter.to_string())])
        )
    }
}

#[tool_handler]
impl ServerHandler for Counter {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            instructions: Some("This server provides a counter tool that can increment, decrement, and get value.".parse().unwrap()),
            ..Default::default()
        }
    }
}