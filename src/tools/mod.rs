pub mod greeting_tool;
pub mod calculator_tool;
pub mod weather_tool;

use serde_json::{Value};
use async_trait::async_trait;
use crate::utils::Result;

/// Represents a tool that can be invoked by the LLM
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Value>,
}

/// Content returned from a tool execution
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TextContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: String,
}

impl TextContent {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            content_type: "text".to_string(),
            text: text.into(),
        }
    }
}

/// Result of a tool call
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CallToolResult {
    pub content: Vec<TextContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isError")]
    pub is_error: Option<bool>,
}

impl CallToolResult {
    pub fn success(content: Vec<TextContent>) -> Self {
        Self {
            content,
            is_error: Some(false),
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            content: vec![TextContent::new(message)],
            is_error: Some(true),
        }
    }
}

/// Trait for implementing tool handlers
#[async_trait]
pub trait ToolHandler: Send + Sync {
    async fn call(&self, arguments: Value) -> Result<CallToolResult>;
}