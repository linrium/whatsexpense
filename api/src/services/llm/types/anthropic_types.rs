use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompletionResponse {
    pub content: Vec<CompletionContent>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum CompletionType {
    Text,
    ToolUse,
}

impl Display for CompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompletionType::Text => write!(f, "text"),
            CompletionType::ToolUse => write!(f, "tool_use"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompletionTextContent {
    pub r#type: CompletionType,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompletionToolUseContent {
    pub r#type: CompletionType,
    pub name: String,
    pub input: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum CompletionContent {
    Text(CompletionTextContent),
    ToolUse(CompletionToolUseContent),
}
