use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestUserMessageArgs, ChatCompletionToolArgs,
    ChatCompletionToolType, CreateChatCompletionRequestArgs, FunctionObject,
};
use async_openai::Client;
use async_trait::async_trait;

use crate::services::llm::{LLMError, LLMServiceExt};

#[derive(Clone)]
pub struct OpenAIService {
    pub client: Client<OpenAIConfig>,
}

#[async_trait]
impl LLMServiceExt for OpenAIService {
    async fn chat_with_fn(
        &self,
        prompt: String,
        fn_obj: FunctionObject,
    ) -> Result<(Vec<String>, String), LLMError> {
        let request = CreateChatCompletionRequestArgs::default()
            .messages(vec![ChatCompletionRequestMessage::User(
                ChatCompletionRequestUserMessageArgs::default()
                    .content(prompt)
                    .build()
                    .unwrap(),
            )])
            .tools(vec![ChatCompletionToolArgs::default()
                .r#type(ChatCompletionToolType::Function)
                .function(fn_obj)
                .build()?])
            .model("gpt-3.5-turbo".to_string())
            .max_tokens(500u32)
            .temperature(0.4)
            .n(1)
            .build()?;

        let response = self.client.chat().create(request).await.unwrap();

        let contents = response
            .clone()
            .choices
            .into_iter()
            .filter_map(|choice| choice.message.tool_calls)
            .flatten()
            .map(|tool_call| tool_call.function.arguments)
            // .map(|arguments| serde_json::from_str(&arguments).unwrap())
            .collect::<Vec<_>>();

        Ok((contents, serde_json::to_string(&response).unwrap()))
    }
}
