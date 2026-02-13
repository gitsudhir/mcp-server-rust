use super::{Tool, CallToolResult, TextContent, ToolHandler};
use serde_json::{json, Value};
use async_trait::async_trait;
use crate::utils::{Result, Error, Logger};

pub struct WeatherTool {
    logger: Logger,
}

impl WeatherTool {
    pub fn new() -> Self {
        Self {
            logger: Logger::new("WeatherTool"),
        }
    }

    pub fn tool_definition() -> Tool {
        Tool {
            name: "fetch-weather".to_string(),
            description: "Fetches weather information for a given city".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "city": {
                        "type": "string",
                        "description": "The city name"
                    }
                },
                "required": ["city"]
            }),
            annotations: Some(json!({
                "title": "Fetch Weather",
                "readOnlyHint": true,
                "openWorldHint": true
            })),
        }
    }
}

impl Default for WeatherTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ToolHandler for WeatherTool {
    async fn call(&self, arguments: Value) -> Result<CallToolResult> {
        let city = arguments
            .get("city")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::InvalidParams("Missing 'city' parameter".to_string()))?;

        self.logger.debug_with_context("Fetching weather for city", city);

        // Simulate weather data (in real scenario, call external API)
        let weather_data = json!({
            "city": city,
            "temperature": "72°F",
            "condition": "Partly Cloudy",
            "humidity": "65%",
            "windSpeed": "10 mph"
        });

        let message = format!(
            "Weather for {}:\n{}",
            city,
            serde_json::to_string_pretty(&weather_data)?
        );

        Ok(CallToolResult::success(vec![TextContent::new(message)]))
    }
}