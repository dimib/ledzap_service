pub mod client;

#[cfg(test)]
mod tests {
    use crate::client::openai_client::OpenaiClient;
    use super::client::model::OpenaiChatMessage;

    #[test]
    fn chat_works() {
        let openai_key = std::env::var("OPENAI_KEY");
        assert!(openai_key.is_ok());
        let client = OpenaiClient::new(openai_key.unwrap().to_string());
        let m1 = OpenaiChatMessage {
            role: "user".to_string(),
            content: "Entschuldigung für vergessene Hausaufgaben.".to_string(),
        };
        let m2 = OpenaiChatMessage {
            role: "user".to_string(),
            content: "Maximal 10 Worte.".to_string(),
        };
        let messages = vec![m1, m2];
        client.chat_blocking(messages).unwrap();
    }
}
