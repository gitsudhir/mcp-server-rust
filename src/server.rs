use crate::tools::*;
use crate::resources::*;
use crate::resources::config_resource::ConfigResource;
use crate::prompts::*;
use crate::prompts::code_review_prompt::CodeReviewPrompt;
use crate::utils::{Result, Error, Logger};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::tools::greeting_tool::GreetingTool;
use crate::tools::calculator_tool::CalculatorTool;
use crate::tools::weather_tool::WeatherTool;
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ServerConfig {
    pub name: String,
    pub version: String,
}

impl ServerConfig {
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
        }
    }
}

pub struct McpServer {
    config: ServerConfig,
    logger: Logger,
    tools: Arc<Mutex<HashMap<String, Arc<dyn ToolHandler>>>>,
    resources: Arc<Mutex<HashMap<String, Arc<dyn ResourceHandler>>>>,
    prompts: Arc<Mutex<HashMap<String, Arc<dyn PromptHandler>>>>,
    initialized: Arc<Mutex<bool>>,
}

impl McpServer {
    pub fn new(config: ServerConfig) -> Self {
        let logger = Logger::new("McpServer");
        logger.info(&format!(
            "Creating MCP server: {} v{}",
            config.name, config.version
        ));

        Self {
            config,
            logger,
            tools: Arc::new(Mutex::new(HashMap::new())),
            resources: Arc::new(Mutex::new(HashMap::new())),
            prompts: Arc::new(Mutex::new(HashMap::new())),
            initialized: Arc::new(Mutex::new(false)),
        }
    }

    pub async fn register_tool(&self, name: String, handler: Arc<dyn ToolHandler>) -> Result<()> {
        self.logger.info(&format!("Registering tool: {}", name));
        self.tools.lock().await.insert(name, handler);
        Ok(())
    }

    pub async fn register_resource(
        &self,
        name: String,
        handler: Arc<dyn ResourceHandler>,
    ) -> Result<()> {
        self.logger.info(&format!("Registering resource: {}", name));
        self.resources.lock().await.insert(name, handler);
        Ok(())
    }

    pub async fn register_prompt(
        &self,
        name: String,
        handler: Arc<dyn PromptHandler>,
    ) -> Result<()> {
        self.logger.info(&format!("Registering prompt: {}", name));
        self.prompts.lock().await.insert(name, handler);
        Ok(())
    }

    pub async fn handle_request(&self, message: Value) -> Result<Option<Value>> {
        // Parse JSON-RPC message
        let jsonrpc = message
            .get("jsonrpc")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::InvalidRequest("Missing jsonrpc field".to_string()))?;

        if jsonrpc != "2.0" {
            return Err(Error::InvalidRequest("Invalid jsonrpc version".to_string()));
        }

        // Check if it's a notification (no id field)
        let id = message.get("id");
        let is_notification = id.is_none();

        let method = message
            .get("method")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::InvalidRequest("Missing method".to_string()))?;

        self.logger.debug(&format!("Handling request: {}", method));

        let result = match method {
            "initialize" => self.handle_initialize(&message).await,
            "initialized" => self.handle_initialized(&message).await,
            "ping" => self.handle_ping(&message).await,
            "tools/list" => self.handle_tools_list(&message).await,
            "tools/call" => self.handle_tools_call(&message).await,
            "resources/list" => self.handle_resources_list(&message).await,
            "resources/read" => self.handle_resources_read(&message).await,
            "prompts/list" => self.handle_prompts_list(&message).await,
            "prompts/get" => self.handle_prompts_get(&message).await,
            _ => Err(Error::MethodNotFound(method.to_string())),
        };

        // Handle result and create response
        if is_notification {
            Ok(None) // No response for notifications
        } else {
            let response = match result {
                Ok(result_value) => {
                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": result_value
                    })
                }
                Err(e) => {
                    let (code, message) = match e {
                        Error::MethodNotFound(_) => (-32601, e.to_string()),
                        Error::InvalidParams(_) => (-32602, e.to_string()),
                        Error::InvalidRequest(_) => (-32600, e.to_string()),
                        _ => (-32603, e.to_string()),
                    };

                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "error": {
                            "code": code,
                            "message": message
                        }
                    })
                }
            };

            Ok(Some(response))
        }
    }

    async fn handle_initialize(&self, _message: &Value) -> Result<Value> {
        self.logger.info("Handling initialize request:");

        let mut initialized = self.initialized.lock().await;
        *initialized = true;
        drop(initialized);

        Ok(json!({
            "protocolVersion": crate::PROTOCOL_VERSION,
            "capabilities": {
                "tools": {},
                "resources": {},
                "prompts": {}
            },
            "serverInfo": {
                "name": self.config.name,
                "version": self.config.version
            }
        }))
    }

    async fn handle_initialized(&self, _message: &Value) -> Result<Value> {
        self.logger.info("Server initialized");
        Ok(json!({}))
    }

    async fn handle_ping(&self, _message: &Value) -> Result<Value> {
        self.logger.debug("Handling ping");
        Ok(json!({}))
    }

    async fn handle_tools_list(&self, _message: &Value) -> Result<Value> {
        self.logger.debug("Listing tools");

        let tools = vec![
            GreetingTool::tool_definition(),
            CalculatorTool::tool_definition(),
            WeatherTool::tool_definition(),
        ];

        Ok(json!({
            "tools": tools
        }))
    }

    async fn handle_tools_call(&self, message: &Value) -> Result<Value> {
        let params = message
            .get("params")
            .ok_or_else(|| Error::InvalidParams("Missing params".to_string()))?;

        let tool_name = params
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::InvalidParams("Missing tool name".to_string()))?;

        let arguments = params
            .get("arguments")
            .cloned()
            .unwrap_or(json!({}));

        self.logger.debug(&format!("Calling tool: {}", tool_name));

        // Match tool by name and call the appropriate handler
        let result = match tool_name {
            "greet" => {
                let handler = GreetingTool::new();
                handler.call(arguments).await?
            }
            "calculate-bmi" => {
                let handler = CalculatorTool::new();
                handler.call(arguments).await?
            }
            "fetch-weather" => {
                let handler = WeatherTool::new();
                handler.call(arguments).await?
            }
            _ => return Err(Error::MethodNotFound(format!("Tool not found: {}", tool_name))),
        };

        Ok(json!(result))
    }

    async fn handle_resources_list(&self, _message: &Value) -> Result<Value> {
        self.logger.debug("Listing resources");

        Ok(json!({
            "resources": [
                {
                    "uri": "config://app",
                    "name": "Application Configuration",
                    "description": "Current application configuration",
                    "mimeType": "application/json"
                }
            ]
        }))
    }

    async fn handle_resources_read(&self, message: &Value) -> Result<Value> {
        let params = message
            .get("params")
            .ok_or_else(|| Error::InvalidParams("Missing params".to_string()))?;

        let uri = params
            .get("uri")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::InvalidParams("Missing resource URI".to_string()))?;

        self.logger.debug(&format!("Reading resource: {}", uri));

        let result = if uri.starts_with("config://") {
            let handler = ConfigResource::new();
            handler.read(uri).await?
        } else {
            return Err(Error::ResourceError(format!("Resource not found: {}", uri)));
        };

        Ok(json!(result))
    }

    async fn handle_prompts_list(&self, _message: &Value) -> Result<Value> {
        self.logger.debug("Listing prompts");

        Ok(json!({
            "prompts": [
                CodeReviewPrompt::prompt_definition()
            ]
        }))
    }

    async fn handle_prompts_get(&self, message: &Value) -> Result<Value> {
        let params = message
            .get("params")
            .ok_or_else(|| Error::InvalidParams("Missing params".to_string()))?;

        let prompt_name = params
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::InvalidParams("Missing prompt name".to_string()))?;

        let arguments = params.get("arguments").cloned();

        self.logger.debug(&format!("Getting prompt: {}", prompt_name));

        let result = match prompt_name {
            "review-code" => {
                let handler = CodeReviewPrompt::new();
                handler.get(arguments).await?
            }
            _ => {
                return Err(Error::MethodNotFound(format!(
                    "Prompt not found: {}",
                    prompt_name
                )))
            }
        };

        Ok(json!(result))
    }
}