/**
 * Just a little Lame Excuse Test Program
 */


use openai_lib::client::model::OpenaiChatMessage;
use openai_lib::client::openai_client::OpenaiClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    println!("Lame Excuse Device - Test");

    let openai_key = std::env::var("OPENAI_KEY");
    if openai_key.is_err() {
        println!("OPENAI_KEY not set");
        return Ok(());
    }

    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 4 {
        println!("Usage: lame_excuse_device <style> <what> <num words>");
        return Ok(());
    }

    let style = args[1].clone();
    let what = args[2].clone();
    let num_words = args[3].clone();

    let m0 = OpenaiChatMessage {
        role: "user".to_string(),
        content: format!("Antworte als {}", style).to_string(),
    };
    let m1 = OpenaiChatMessage {
        role: "user".to_string(),
        content: format!("Faule Ausrede für {}", what).to_string(),
    };
    let m2 = OpenaiChatMessage {
        role: "user".to_string(),
        content: format!("Maximal {} Worte", num_words).to_string(),
    };
    let messages = vec![m0, m1, m2];

    messages.iter().for_each(|m| println!("{:?}", m.content));

    let client = OpenaiClient::new(openai_key.unwrap().to_string());
    let result = client.chat(messages, 50).await.unwrap();

    println!("{:?}", result.choices[0].message.content);

    Ok(())
}
