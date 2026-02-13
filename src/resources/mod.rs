pub mod config_resource;
pub mod file_resource;

use async_trait::async_trait;
use crate::utils::Result;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Resource {
    pub uri: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob: Option<String>, // base64 encoded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ResourceReadResult {
    pub contents: Vec<Resource>,
}

#[async_trait]
pub trait ResourceHandler: Send + Sync {
    async fn read(&self, uri: &str) -> Result<ResourceReadResult>;
}