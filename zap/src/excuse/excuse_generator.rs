
/**
 * Excuse Generator
 */
use serde::ser::StdError;
use openai_lib::client::model::OpenaiChatMessage;
use openai_lib::client::openai_client::OpenaiClient;

use super::api_keys::openai_key;

pub async fn generate_excuse(persona: String, topic: String, num_words: i64) -> Result<String, Box<dyn StdError>> {
    let key = openai_key();
    let client = OpenaiClient::new(key.to_string());

    let m0 = OpenaiChatMessage {
        role: "user".to_string(),
        content: format!("Antworte als {}", persona).to_string(),
    };
    let m1 = OpenaiChatMessage {
        role: "user".to_string(),
        content: format!("Schreibe faule Ausrede für {}", topic).to_string(),
    };
    let m2 = OpenaiChatMessage {
        role: "user".to_string(),
        content: format!("Maximal {} Worte", num_words).to_string(),
    };
    let messages = vec![m0, m1, m2];

    match client.chat(messages, num_words).await {
        Ok(response) => {
            let excuse = response.choices[0].message.content.clone();
            return Ok(excuse);
        },
        Err(error) => {
            return Err(error);
        }
    }
}