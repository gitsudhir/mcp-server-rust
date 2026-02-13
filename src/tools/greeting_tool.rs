use super::{Tool, CallToolResult, TextContent, ToolHandler};
use serde_json::{json, Value};
use async_trait::async_trait;
use crate::utils::{Result, Error, Logger};

pub struct GreetingTool {
    logger: Logger,
}

impl GreetingTool {
    pub fn new() -> Self {
        Self {
            logger: Logger::new("GreetingTool"),
        }
    }

    pub fn tool_definition() -> Tool {
        Tool {
            name: "greet".to_string(),
            description: "Greets a person with a friendly message".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "The name of the person to greet"
                    }
                },
                "required": ["name"]
            }),
            annotations: Some(json!({
                "title": "Greet Tool",
                "readOnlyHint": true
            })),
        }
    }
}

impl Default for GreetingTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ToolHandler for GreetingTool {
    async fn call(&self, arguments: Value) -> Result<CallToolResult> {
        let name = arguments
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::InvalidParams("Missing 'name' parameter".to_string()))?;

        self.logger.debug_with_context("Tool called with name", name);

        let message = format!("Hello, {}! Welcome to MCP.", name);
        Ok(CallToolResult::success(vec![TextContent::new(message)]))
    }
}