pub mod stdio;

pub use stdio::StdioTransport;

use async_trait::async_trait;
use crate::utils::Result;

#[async_trait]
pub trait Transport: Send + Sync {
    async fn send(&mut self, message: serde_json::Value) -> Result<()>;
    async fn receive(&mut self) -> Result<Option<serde_json::Value>>;
    async fn close(&mut self) -> Result<()>;
}