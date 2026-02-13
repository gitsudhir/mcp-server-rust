pub mod code_review_prompt;

use serde_json::Value;
use async_trait::async_trait;
use crate::utils::Result;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Message {
    pub role: String,
    pub content: Vec<MessageContent>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MessageContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: String,
}

impl MessageContent {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            content_type: "text".to_string(),
            text: text.into(),
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GetPromptResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub messages: Vec<Message>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Prompt {
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<PromptArgument>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PromptArgument {
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

#[async_trait]
pub trait PromptHandler: Send + Sync {
    async fn get(&self, arguments: Option<Value>) -> Result<GetPromptResult>;
}