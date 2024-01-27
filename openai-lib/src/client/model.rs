use serde::{Serialize, Deserialize};

// -----------------------
// Request
// -----------------------
#[derive(Debug, Serialize)]
pub struct OpenaiChatRequest {
    pub model: String,
    pub messages: Vec<OpenaiChatMessage>,
    pub max_tokens: Option<i64>,
    pub temperature: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenaiChatMessage {
    pub role: String,
    pub content: String,
}

// -----------------------
// Response
// -----------------------
#[derive(Debug, Deserialize)]
pub struct OpenaiChatResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<OpenaiChatResponseChoice>,
    pub usage: OpenaiChatUsage,
}

#[derive(Debug, Deserialize)]
pub struct OpenaiChatResponseChoice {
    pub index: i64,
    pub message: OpenaiChatMessage,
    pub finish_reason: String,
}

#[derive(Debug, Deserialize)]
pub struct OpenaiChatUsage {
    pub prompt_tokens: i64,
    pub completion_tokens: i64,
    pub total_tokens: i64,
}