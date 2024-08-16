use crate::services::llm::LLMError;
use async_openai::types::FunctionObject;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait LLMServiceExt: Send + Sync {
    async fn chat_with_fn(
        &self,
        prompt: String,
        fn_obj: FunctionObject,
    ) -> Result<(Vec<String>, String), LLMError>;
}

pub type LLMServiceDyn = Arc<dyn LLMServiceExt + Send + Sync>;
