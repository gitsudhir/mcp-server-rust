use crate::utils::{Result, Logger};
use serde_json::json;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use futures::future::BoxFuture;
use crate::transport::Transport;
/// Standard Input/Output transport for MCP servers
/// 
/// Messages are sent as newline-delimited JSON-RPC 2.0 messages
/// through stdin/stdout with logging to stderr.
pub struct StdioTransport {
    logger: Logger,
    // For testing and flexibility, we use in-memory buffers wrapped in Arc<Mutex>
    reader: Arc<Mutex<tokio::io::BufReader<tokio::io::Stdin>>>,
    writer: Arc<Mutex<tokio::io::Stdout>>,
}

impl StdioTransport {
    pub fn new() -> Self {
        let logger = Logger::new("StdioTransport");
        logger.info("Initializing StdioTransport");

        Self {
            logger,
            reader: Arc::new(Mutex::new(tokio::io::BufReader::new(tokio::io::stdin()))),
            writer: Arc::new(Mutex::new(tokio::io::stdout())),
        }
    }

    pub async fn listen<F>(
        &mut self,
        handler: F,
    ) -> Result<()>
    where
        F: Fn(serde_json::Value) -> BoxFuture<'static, Result<Option<serde_json::Value>>> + Send + Sync + 'static,
    {
        self.logger.info("Starting to listen on stdio");

        loop {
            match self.receive().await {
                Ok(Some(message)) => {
                    self.logger.debug_with_context("Received message", &message.to_string());

                    match handler(message.clone()).await {
                        Ok(Some(response)) => {
                            self.logger.debug_with_context("Sending response", &response.to_string());
                            self.send(response).await?;
                        }
                        Ok(None) => {
                            // Notification; no response needed
                            self.logger.debug("Notification processed, no response sent");
                        }
                        Err(e) => {
                            self.logger.error_with_context("Handler error", &e.to_string());
                            if let Some(id) = message.get("id") {
                                let error_response = json!({
                                    "jsonrpc": "2.0",
                                    "id": id,
                                    "error": {
                                        "code": -32603,
                                        "message": "Internal error",
                                        "data": e.to_string()
                                    }
                                });
                                self.send(error_response).await?;
                            }
                        }
                    }
                }
                Ok(None) => {
                    self.logger.info("Stdin closed, shutting down");
                    break;
                }
                Err(e) => {
                    self.logger.error(&format!("Transport error: {}", e));
                    break;
                }
            }
        }

        Ok(())
    }
}

impl Default for StdioTransport {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl super::Transport for StdioTransport {
    async fn send(&mut self, message: serde_json::Value) -> Result<()> {
        let json_str = serde_json::to_string(&message)?;
        let mut writer = self.writer.lock().await;
        writer.write_all(json_str.as_bytes()).await?;
        writer.write_all(b"\n").await?;
        writer.flush().await?;
        Ok(())
    }

    async fn receive(&mut self) -> Result<Option<serde_json::Value>> {
        let mut reader = self.reader.lock().await;
        let mut line = String::new();
        match reader.read_line(&mut line).await? {
            0 => Ok(None), // EOF
            _ => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    return Ok(None);
                }
                let json = serde_json::from_str(trimmed)?;
                Ok(Some(json))
            }
        }
    }

    async fn close(&mut self) -> Result<()> {
        self.logger.info("Closing StdioTransport");
        Ok(())
    }
}