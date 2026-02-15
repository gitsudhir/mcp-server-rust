use super::{GetPromptResult, Message, MessageContent, Prompt, PromptArgument, PromptHandler};
use crate::utils::{Error, Logger, Result};
use async_trait::async_trait;
use serde_json::Value;

pub struct CodeReviewPrompt {
    logger: Logger,
}

impl CodeReviewPrompt {
    pub fn new() -> Self {
        Self {
            logger: Logger::new("CodeReviewPrompt"),
        }
    }

    pub fn prompt_definition() -> Prompt {
        Prompt {
            name: "review-code".to_string(),
            description: "Generates a prompt to ask the LLM to review code".to_string(),
            arguments: Some(vec![
                PromptArgument {
                    name: "code".to_string(),
                    description: "The code snippet to review".to_string(),
                    required: Some(true),
                },
                PromptArgument {
                    name: "focus".to_string(),
                    description: "Optional area of focus for the review (performance, security, style, general)".to_string(),
                    required: Some(false),
                },
            ]),
        }
    }
}

impl Default for CodeReviewPrompt {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PromptHandler for CodeReviewPrompt {
    async fn get(&self, arguments: Option<Value>) -> Result<GetPromptResult> {
        let args =
            arguments.ok_or_else(|| Error::InvalidParams("Missing arguments".to_string()))?;

        let code = args
            .get("code")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::InvalidParams("Missing 'code' argument".to_string()))?;

        let focus = args
            .get("focus")
            .and_then(|v| v.as_str())
            .unwrap_or("general");

        self.logger
            .debug_with_context("Generating code review prompt", focus);

        let mut prompt_text =
            "Please review the following code for potential issues and suggest improvements"
                .to_string();
        if focus != "general" {
            prompt_text.push_str(&format!(", focusing specifically on {}", focus));
        }
        prompt_text.push_str(&format!(":\n\n```\n{}\n```", code));

        Ok(GetPromptResult {
            description: Some(format!("Requesting {} review for code snippet", focus)),
            messages: vec![Message {
                role: "user".to_string(),
                content: vec![MessageContent::new(prompt_text)],
            }],
        })
    }
}
