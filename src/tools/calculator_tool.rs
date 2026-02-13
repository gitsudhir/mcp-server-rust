use super::{Tool, CallToolResult, TextContent, ToolHandler};
use serde_json::{json, Value};
use async_trait::async_trait;
use crate::utils::{Result, Error, Logger};

pub struct CalculatorTool {
    logger: Logger,
}

impl CalculatorTool {
    pub fn new() -> Self {
        Self {
            logger: Logger::new("CalculatorTool"),
        }
    }

    pub fn tool_definition() -> Tool {
        Tool {
            name: "calculate-bmi".to_string(),
            description: "Calculates Body Mass Index from weight and height".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "weightKg": {
                        "type": "number",
                        "description": "Weight in kilograms"
                    },
                    "heightM": {
                        "type": "number",
                        "description": "Height in meters",
                        "minimum": 0.1
                    }
                },
                "required": ["weightKg", "heightM"]
            }),
            annotations: Some(json!({
                "title": "BMI Calculator",
                "readOnlyHint": true
            })),
        }
    }
}

impl Default for CalculatorTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ToolHandler for CalculatorTool {
    async fn call(&self, arguments: Value) -> Result<CallToolResult> {
        let weight_kg = arguments
            .get("weightKg")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| Error::InvalidParams("Missing or invalid 'weightKg'".to_string()))?;

        let height_m = arguments
            .get("heightM")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| Error::InvalidParams("Missing or invalid 'heightM'".to_string()))?;

        if height_m <= 0.0 {
            return Ok(CallToolResult::error("Height must be positive"));
        }

        self.logger.debug_with_context(
            "Calculating BMI",
            &format!("weight={}kg, height={}m", weight_kg, height_m),
        );

        let bmi = weight_kg / (height_m * height_m);
        let message = format!("BMI: {:.2}", bmi);

        Ok(CallToolResult::success(vec![TextContent::new(message)]))
    }
}