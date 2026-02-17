use super::{Resource, ResourceReadResult, ResourceHandler, ResourceDefinition};
use serde_json::{json};
use async_trait::async_trait;
use crate::utils::{Result, Logger, Error};

pub struct ConfigResource {
    logger: Logger,
}

impl ConfigResource {
    pub fn new() -> Self {
        Self {
            logger: Logger::new("ConfigResource"),
        }
    }
}

impl Default for ConfigResource {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ResourceHandler for ConfigResource {
    async fn read(&self, uri: &str) -> Result<ResourceReadResult> {
        self.logger.debug_with_context("Reading config resource", uri);

        let config_data = json!({
            "appName": "Rust MCP Server",
            "version": "1.0.0",
            "environment": "development",
            "features": {
                "tools": true,
                "resources": true,
                "prompts": true
            }
        });

        let content = serde_json::to_string_pretty(&config_data)?;

        Ok(ResourceReadResult {
            contents: vec![Resource {
                uri: uri.to_string(),
                mime_type: "application/json".to_string(),
                text: Some(content),
                blob: None,
                size: None,
                is_dir: Some(false),
            }],
        })
    }

    async fn write(&self, uri: &str, _content: &[u8], _mime_type: &str) -> Result<()> {
        self.logger.warn_with_context("Attempted to write to config resource", uri);
        Err(Error::ResourceError("Writing to config resources is not supported".to_string()))
    }

    async fn list_directory(&self, uri: &str) -> Result<ResourceReadResult> {
        self.logger.warn_with_context("Attempted to list config resource as directory", uri);
        Err(Error::ResourceError("Config resources are not directories".to_string()))
    }

    fn resource_definition(&self) -> ResourceDefinition {
        ResourceDefinition {
            uri: "config://app".to_string(),
            name: "Application Configuration".to_string(),
            description: "Current application configuration".to_string(),
            mime_type: "application/json".to_string(),
        }
    }
}