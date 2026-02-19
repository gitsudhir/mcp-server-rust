use super::{Tool, CallToolResult, TextContent, ToolHandler};
use serde_json::{json, Value};
use async_trait::async_trait;
use crate::utils::{Result, Error, Logger};
use reqwest::Client;

pub struct WebTool {
    logger: Logger,
    http_client: Client,
}

impl WebTool {
    pub fn new() -> Self {
        Self {
            logger: Logger::new("WebTool"),
            http_client: Client::new(),
        }
    }

    pub fn tool_definition() -> Tool {
        Tool {
            name: "fetch-url".to_string(),
            description: "Fetches a URL and returns its content as text".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "url": {
                        "type": "string",
                        "description": "The URL to fetch content from"
                    }
                },
                "required": ["url"]
            }),
            annotations: Some(json!({
                "title": "Fetch URL Content",
                "readOnlyHint": true,
                "openWorldHint": true
            })),
        }
    }
}

impl Default for WebTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ToolHandler for WebTool {
    async fn call(&self, arguments: Value) -> Result<CallToolResult> {
        let url = arguments
            .get("url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::InvalidParams("Missing 'url' parameter".to_string()))?;

        self.logger.debug_with_context("Fetching URL content", url);

        // Perform HTTP GET request
        let response = self.http_client
            .get(url)
            .send()
            .await
            .map_err(|e| Error::NetworkError(format!("Network request failed: {}", e)))?;

        // Read the response body as text
        let body_text = response
            .text()
            .await
            .map_err(|e| Error::NetworkError(format!("Failed to read response text: {}", e)))?;

        // Return the result as a text content
        Ok(CallToolResult::success(vec![
            TextContent::new(body_text)
        ]))
    }
}
