
use rocket::serde::json::{Value, json};
use rocket::http::Status;
//use rocket::State;
use rocket::Request;

use crate::excuse::model::{ExcuseResponse, ExcuseRequest};
use crate::excuse::api_keys::ApiKey;
use super::excuse_generator;


/**
 * This is the handler for the GET /excuse endpoint.
 */
#[get("/?<persona>&<topic>&<num_words>", format = "json")]
pub async fn get_excuse_as_json(_key: ApiKey<'_>, persona: Option<String>, topic: Option<String>, num_words: Option<i64>) -> (Status, Value) {

    let persona = match persona {
        Some(p) => p,
        None => "normaler Typ".to_string(),
    };

    let topic = match topic {
        Some(t) => t,
        None => "Hausaufgaben".to_string(),
    };

    let num_words = match num_words {
        Some(n) => n,
        None => 50,
    };

    let result = excuse_generator::generate_excuse(persona.clone(), topic.clone(), num_words).await;
    match result {
        Ok(excuse) => {
            return (Status::Ok, json!(ExcuseResponse {
                status: "OK".to_string(),
                excuse: excuse,
            }))
        },
        Err(_error) => {
            return (Status::Ok, json!(ExcuseResponse {
                status: "ERROR".to_string(),
                excuse: format!("Entschuldige,ich war beschäftigt damit, eine Ausrede für {} als {} mit {} zu finden!", topic, persona, num_words).to_string(),
            }))
        }
    }
}

/**
 * This is the handler for the POST /excuse endpoint.
 */
#[post("/", format = "json", data = "<excuse_request>")]
pub async fn post_excuse_as_json(excuse_request: ExcuseRequest) -> (Status, Value) {

    let num_words = match excuse_request.num_words {
        Some(n) => n,
        None => 50,
    };

    let persona = excuse_request.persona.clone();
    let topic = excuse_request.topic.clone();

    let result = excuse_generator::generate_excuse(persona.clone(), topic.clone(), num_words).await;
    match result {
        Ok(excuse) => {
            return (Status::Ok, json!(ExcuseResponse {
                status: "OK".to_string(),
                excuse: excuse,
            }))
        },
        Err(_error) => {
            return (Status::Ok, json!(ExcuseResponse {
                status: "ERROR".to_string(),
                excuse: format!("Entschuldige,ich war beschäftigt damit, eine Ausrede für {} als {} mit {} zu finden!", topic, persona, num_words).to_string(),
            }))
        }
    }
}

#[catch(401)]
pub fn not_unauthorized(_req: &Request) -> String {
    format!("Not authorized")
}

#[catch(422)]
pub fn unprocessable_entity(_req: &Request) -> String {
    format!("Unprocessable entity")
}