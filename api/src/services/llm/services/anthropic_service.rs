use async_openai::types::FunctionObject;
use async_trait::async_trait;
use serde_json::json;
use tracing::debug;

use crate::services::llm::types::anthropic_types::{CompletionContent, CompletionResponse};
use crate::services::llm::{LLMError, LLMServiceExt};

pub struct AnthropicService {
    pub http_client: reqwest::Client,
    pub api_key: String,
}

#[async_trait]
impl LLMServiceExt for AnthropicService {
    async fn chat_with_fn(
        &self,
        prompt: String,
        fn_obj: FunctionObject,
    ) -> Result<(Vec<String>, String), LLMError> {
        debug!("prompt: {prompt}");
        let completion = self
            .http_client
            .post("https://api.anthropic.com/v1/messages")
            .header("Content-Type", "application/json")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&json!({
                "model": "claude-3-haiku-20240307",
                "max_tokens": 1800,
                "messages": [
                    {
                        "role": "user",
                        "content": prompt
                    }
                ],
                "tools": [
                    {
                        "name": fn_obj.name,
                        "description": fn_obj.description,
                        "input_schema": fn_obj.parameters
                    }
                ]
            }))
            .send()
            .await
            .map_err(|e| LLMError::Unknown(e.into()))?
            .text()
            .await
            .map_err(|e| LLMError::Unknown(e.into()))?;

        debug!("completion: {:?}", completion);
        let response = serde_json::from_str::<CompletionResponse>(&completion)
            .map_err(|e| LLMError::Unknown(e.into()))?;
        let contents = response
            .content
            .into_iter()
            .filter_map(|content| match content {
                CompletionContent::Text(_) => None,
                CompletionContent::ToolUse(tool_use_content) => Some(tool_use_content.input),
            })
            .flat_map(|input| {
                serde_json::to_string(&input).map_err(|e| LLMError::Unknown(e.into()))
            })
            .collect::<Vec<_>>();

        debug!("contents: {:?}", contents);

        Ok((contents, completion))
    }
}
