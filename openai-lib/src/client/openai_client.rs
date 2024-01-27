
use crate::client::model::{OpenaiChatRequest, OpenaiChatMessage, OpenaiChatResponse};
pub struct OpenaiClient {
    api_key: String,
    model: String,
    url: String,
}

impl OpenaiClient {
    pub fn new(api_key: String) -> OpenaiClient {
        OpenaiClient {
            api_key,
            model: "gpt-3.5-turbo-1106".to_string(),
            url: "https://api.openai.com/v1/chat/completions".to_string(),
        }
    }

    pub fn chat_blocking(&self, messages: Vec<OpenaiChatMessage>) -> Result<OpenaiChatResponse, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        let request = OpenaiChatRequest {
            model: self.model.clone(),
            messages,
            max_tokens: None,
            temperature: Some(1.0),
        };
        let response = client.post(&self.url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()?
            .json::<OpenaiChatResponse>()?;
        Ok(response)
    }

    pub async fn chat(&self, messages: Vec<OpenaiChatMessage>, _max_tokens: i64) -> Result<OpenaiChatResponse, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let request = OpenaiChatRequest {
            model: self.model.clone(),
            messages,
            max_tokens: None,
            temperature: Some(1.0),
        };
        let response = client.post(&self.url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?
            .json::<OpenaiChatResponse>()
            .await?;
        Ok(response)
    }
}

