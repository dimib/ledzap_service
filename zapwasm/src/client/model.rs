use serde::{ Deserialize, Serialize };

// use super::api_keys;

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