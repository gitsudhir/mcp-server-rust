use super::{Resource, ResourceReadResult, ResourceHandler, ResourceDefinition};
use async_trait::async_trait;
use crate::utils::{Result, Error, Logger};
use std::path::{PathBuf};
use tokio::fs;

pub struct FileResource {
    logger: Logger,
    base_dir: PathBuf,
}

impl FileResource {
    pub fn new(base_dir: PathBuf) -> Self {
        Self {
            logger: Logger::new("FileResource"),
            base_dir,
        }
    }

    fn validate_path(&self, filename: &str) -> Result<PathBuf> {
        let requested_path = self.base_dir.join(filename);
        let resolved_base = std::fs::canonicalize(&self.base_dir)
            .unwrap_or_else(|_| self.base_dir.clone());
        let resolved_requested = std::fs::canonicalize(&requested_path)
            .unwrap_or_else(|_| requested_path.clone());

        if !resolved_requested.starts_with(&resolved_base) {
            return Err(Error::ResourceError(
                "Access denied: Path traversal attempt".to_string(),
            ));
        }

        Ok(resolved_requested)
    }
}

#[async_trait]
impl ResourceHandler for FileResource {
    async fn read(&self, uri: &str) -> Result<ResourceReadResult> {
        // Parse URI: file:///data/{filename}
        let filename = uri
            .strip_prefix("file:///data/")
            .ok_or_else(|| Error::ResourceError(format!("Invalid URI: {}", uri)))?;

        self.logger.debug_with_context("Reading file resource", filename);

        let validated_path = self.validate_path(filename)?;

        match fs::read_to_string(&validated_path).await {
            Ok(content) => {
                let mime_type = if filename.ends_with(".txt") {
                    "text/plain"
                } else if filename.ends_with(".json") {
                    "application/json"
                } else {
                    "application/octet-stream"
                };

                Ok(ResourceReadResult {
                    contents: vec![Resource {
                        uri: uri.to_string(),
                        mime_type: mime_type.to_string(),
                        text: Some(content),
                        blob: None,
                        size: None,
                        is_dir: Some(false), // Set is_dir to false for files
                    }],
                })
            }
            Err(e) => {
                self.logger.error_with_context("File read error", &e.to_string());
                Err(Error::ResourceError(format!("Failed to read file: {}", e)))
            }
        }
    }

    async fn write(&self, uri: &str, content: &[u8], _mime_type: &str) -> Result<()> {
        let filename = uri
            .strip_prefix("file:///data/")
            .ok_or_else(|| Error::ResourceError(format!("Invalid URI: {}", uri)))?;

        self.logger.debug_with_context("Writing file resource", filename);

        let validated_path = self.validate_path(filename)?;

        match fs::write(&validated_path, content).await {
            Ok(_) => Ok(()),
            Err(e) => {
                self.logger.error_with_context("File write error", &e.to_string());
                Err(Error::ResourceError(format!("Failed to write file: {}", e)))
            }
        }
    }

    async fn list_directory(&self, uri: &str) -> Result<ResourceReadResult> {
        let dirname = uri
            .strip_prefix("file:///data/")
            .ok_or_else(|| Error::ResourceError(format!("Invalid URI: {}", uri)))?;

        self.logger.debug_with_context("Listing directory resource", dirname);

        let validated_path = self.validate_path(dirname)?;

        let mut entries = fs::read_dir(&validated_path).await?;
        let mut resources = Vec::new();

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            let file_name = path.file_name()
                                .and_then(|name| name.to_str())
                                .unwrap_or("unknown");
            let entry_uri = format!("{}/{}", uri.trim_end_matches('/'), file_name);

            let metadata = entry.metadata().await?;
            let is_dir = metadata.is_dir();
            let size = metadata.len();

            resources.push(Resource {
                uri: entry_uri,
                mime_type: if is_dir { "inode/directory".to_string() } else { "application/octet-stream".to_string() },
                text: None,
                blob: None,
                size: Some(size),
                is_dir: Some(is_dir),
            });
        }

        Ok(ResourceReadResult { contents: resources })
    }

    fn resource_definition(&self) -> ResourceDefinition {
        ResourceDefinition {
            uri: "file:///data".to_string(),
            name: "File System".to_string(),
            description: "Provides file system operations: read, write, and list directory.".to_string(),
            mime_type: "application/octet-stream".to_string(),
        }
    }
}