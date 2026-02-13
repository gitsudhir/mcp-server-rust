//! Model Context Protocol (MCP) Server in Rust
//!
//! A complete implementation of an MCP stdio server with tools, resources, and prompts.

use mcp_server_rust::{
    McpServer, ServerConfig,
    utils::logger::init_logger,
    transport::StdioTransport,
};
use tracing::error;
use futures::future::BoxFuture;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging to stderr
    init_logger();

    // Create server configuration
    let config = ServerConfig::new("RustMcpServer", "1.0.0");

    // Create MCP server instance
    let server = McpServer::new(config);

    // Create stdio transport
    let mut transport = StdioTransport::new();

    // Create a handler closure that processes JSON-RPC messages
    let server_arc = std::sync::Arc::new(server);
    let handler = {
        let server = server_arc.clone();
        move |message: serde_json::Value| -> BoxFuture<'static, mcp_server_rust::utils::Result<Option<serde_json::Value>>> {
            let server = server.clone();
            Box::pin(async move {
                server.handle_request(message).await
            })
        }
    };

    // Start listening on stdio
    if let Err(e) = transport.listen(handler).await {
        error!("Transport error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}