use rocket::futures::FutureExt;
use serde::{ Deserialize, Serialize };

use rocket::{Request, Data};
use rocket::data::{FromData, Outcome, ToByteUnit};
use rocket::http::Status;

use super::api_keys;

/**
 * This is the requested excuse. 
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct ExcuseResponse {
    // Excuse status:
    // "OK" = excuse generated,
    // "ERROR" = error generating excuse
    pub status: String,

    // The excuse message. If status is "ERROR", the "excuse" will
    // contain an excuse for the service itself 😏
    pub excuse: String,
}

/**
 * The excuse request contains all information needed to generate
 * an excuse.
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct ExcuseRequest {
    // The persona that generates the excuse. This could be a person
    // like "Rapper", "Politician", "Developer", "Manager", etc.
    pub persona: String,

    // The number of words for the excuse.
    pub num_words: Option<i64>,

    // The length expected length of the excuse. Could be
    // "short", "medium", "long", etc.
    pub length: Option<String>,

    // The topic of the excuse. This could be a topic like "Homework",
    // "Businesplan", "Project", "Meeting", "Presentation", etc.
    pub topic: String,
}

#[derive(Debug)]
pub enum ExcuseError {
    NotAllowed,
    NotAvailable,
}

const DATA_LIMIT: i32 = 1024;

#[rocket::async_trait]
impl<'r> FromData<'r> for ExcuseRequest {
    type Error = ExcuseError;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {

        let user_token = req.headers().get_one("x-api-key").unwrap_or("");
        if user_token != api_keys::api_key() {
            return Outcome::Error((Status::Unauthorized, ExcuseError::NotAllowed));
        }

        data.open(DATA_LIMIT.kibibytes()).into_string().map(|result| {
            match result {
                Ok(capped) => {
                    let excuse_request: ExcuseRequest = serde_json::from_str(&capped).unwrap();
                    Outcome::Success(excuse_request)
                },
                Err(_) => Outcome::Error((Status::BadRequest, ExcuseError::NotAvailable))
            }
        }).await
    }
}